use super::networking_state::{ListenerState, TcpSettings, TcpState, UdpState};
use super::{
    Capability, CapabilityInner, ResultValue, cancellation_error, cancellation_wait, count,
    is_cancelled, networking_state, operation_deadline, readiness_operation, timeout,
};
use std::net::{IpAddr, Shutdown, SocketAddr, TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, LazyLock};
use std::time::Duration;

// Synchronous networking is supported off-runtime or via block_in_place on a
// multithread executor. A current-thread executor must delegate blocking work
// explicitly: blocking its sole reactor thread would deadlock readiness waits.
fn network_block_on<F: std::future::Future>(future: F) -> Result<F::Output, ResultValue> {
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        if handle.runtime_flavor() == tokio::runtime::RuntimeFlavor::MultiThread {
            return Ok(tokio::task::block_in_place(|| handle.block_on(future)));
        }
        return Err(ResultValue::error(
            "network operation requires explicit blocking delegation on the local executor",
        ));
    }
    Ok(network_runtime()?.block_on(future))
}

fn network_runtime() -> Result<&'static tokio::runtime::Runtime, ResultValue> {
    static RUNTIME: LazyLock<Result<tokio::runtime::Runtime, String>> = LazyLock::new(|| {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .map_err(|error| format!("cannot start network runtime: {error}"))
    });
    RUNTIME
        .as_ref()
        .map_err(|error| ResultValue::error(error.clone()))
}

fn dns_resolver() -> Result<&'static hickory_resolver::TokioResolver, ResultValue> {
    static RESOLVER: LazyLock<Result<hickory_resolver::TokioResolver, String>> =
        LazyLock::new(|| {
            hickory_resolver::TokioResolver::builder_tokio()
                .map(hickory_resolver::ResolverBuilder::build)
                .map_err(|error| format!("cannot configure DNS resolver: {error}"))
        });
    RESOLVER
        .as_ref()
        .map_err(|error| ResultValue::error(error.clone()))
}

pub(super) fn io_error(operation: &str, error: &std::io::Error) -> ResultValue {
    if matches!(
        error.kind(),
        std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock
    ) {
        ResultValue {
            failed: true,
            deadline_exceeded: true,
            message: format!("{operation} deadline exceeded"),
            ..ResultValue::default()
        }
    } else {
        ResultValue::error(format!("{operation} failed: {error}"))
    }
}

pub fn tcp_bind(
    address: &str,
    backlog: i128,
    reuse_address: bool,
    accepted_no_delay: bool,
    accepted_ttl: i128,
) -> ResultValue {
    let Ok(address) = address.parse::<SocketAddr>() else {
        return ResultValue::error("invalid socket address");
    };
    let Some(backlog) = i32::try_from(backlog).ok().filter(|value| *value >= 0) else {
        return ResultValue::error("TCP backlog must be in 0..=2147483647");
    };
    let ttl = match tcp_ttl(accepted_ttl) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let domain = if address.is_ipv4() {
        socket2::Domain::IPV4
    } else {
        socket2::Domain::IPV6
    };
    let socket =
        match socket2::Socket::new(domain, socket2::Type::STREAM, Some(socket2::Protocol::TCP)) {
            Ok(socket) => socket,
            Err(error) => return io_error("TCP socket creation", &error),
        };
    if let Err(error) = socket.set_reuse_address(reuse_address) {
        return io_error("TCP reuse-address configuration", &error);
    }
    if let Err(error) = socket
        .bind(&address.into())
        .and_then(|()| socket.listen(backlog))
    {
        return io_error("TCP bind", &error);
    }
    let listener: TcpListener = socket.into();
    let text = listener
        .local_addr()
        .map_or_else(|_| String::new(), |value| value.to_string());
    if let Err(error) = listener.set_nonblocking(true) {
        return io_error("TCP listener configuration", &error);
    }
    ResultValue {
        text,
        capability: Some(Capability(Arc::new(CapabilityInner::Listener(
            ListenerState::new(
                listener,
                TcpSettings {
                    no_delay: accepted_no_delay,
                    ttl,
                },
            ),
        )))),
        ..ResultValue::default()
    }
}
fn tcp_ttl(ttl: i128) -> Result<Option<u32>, ResultValue> {
    if ttl == -1 {
        return Ok(None);
    }
    u32::try_from(ttl)
        .map(Some)
        .map_err(|_| ResultValue::error("TCP TTL must be -1 (unchanged) or in 0..=4294967295"))
}

fn configure_connected_tcp(
    stream: &TcpStream,
    no_delay: bool,
    ttl: i128,
) -> Result<(), ResultValue> {
    TcpSettings {
        no_delay,
        ttl: tcp_ttl(ttl)?,
    }
    .apply_std(stream)
}
pub fn tcp_connect(
    address: &str,
    no_delay: bool,
    ttl: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let Ok(address) = address.parse::<SocketAddr>() else {
        return ResultValue::error("invalid socket address");
    };
    let duration = match timeout(deadline_nanos) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let connected = duration.map_or_else(
        || TcpStream::connect(address),
        |duration| TcpStream::connect_timeout(&address, duration),
    );
    match connected {
        Ok(stream) => {
            if let Err(error) = configure_connected_tcp(&stream, no_delay, ttl) {
                return error;
            }
            if let Err(error) = stream.set_read_timeout(duration) {
                return io_error("TCP read timeout configuration", &error);
            }
            if let Err(error) = stream.set_write_timeout(duration) {
                return io_error("TCP write timeout configuration", &error);
            }
            ResultValue {
                capability: Some(Capability(Arc::new(CapabilityInner::Tcp(TcpState::new(
                    stream,
                ))))),
                ..ResultValue::default()
            }
        }
        Err(error) => io_error("TCP connect", &error),
    }
}

pub fn tcp_connect_host(
    host: &str,
    port: i128,
    no_delay: bool,
    ttl: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let duration = match timeout(deadline_nanos) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let Ok(port) = u16::try_from(port) else {
        return ResultValue::error("port must be in 0..=65535");
    };
    let started = std::time::Instant::now();
    let resolved = dns_lookup(host, i128::from(port), deadline_nanos, cancellation);
    if resolved.failed {
        return resolved;
    }
    let candidates = resolved
        .entries
        .iter()
        .filter_map(|entry| entry.parse::<SocketAddr>().ok())
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        return ResultValue::error("DNS lookup returned no usable addresses");
    }
    let remaining = duration.map(|duration| duration.saturating_sub(started.elapsed()));
    if remaining.is_some_and(|duration| duration.is_zero()) {
        return ResultValue {
            failed: true,
            deadline_exceeded: true,
            message: "TCP connect deadline exceeded".to_owned(),
            ..ResultValue::default()
        };
    }
    let outcome = match network_block_on(async {
        let mut tasks = tokio::task::JoinSet::new();
        for (index, address) in candidates.into_iter().enumerate() {
            tasks.spawn(async move {
                tokio::time::sleep(Duration::from_millis((index as u64).saturating_mul(250))).await;
                tokio::net::TcpStream::connect(address)
                    .await
                    .ok()
                    .map(|stream| (stream, address))
            });
        }
        let race = async {
            loop {
                tokio::select! {
                    joined = tasks.join_next() => match joined {
                        Some(Ok(Some(value))) => break Ok(value),
                        Some(Ok(None) | Err(_)) => {}
                        None => break Err(ResultValue::error("all TCP connection candidates failed")),
                    },
                    () = tokio::time::sleep(Duration::from_millis(10)) => {
                        if cancellation.is_some_and(|token| is_cancelled(token).unwrap_or(true)) {
                            break Err(ResultValue::error("operation cancelled"));
                        }
                    }
                }
            }
        };
        let result = match if let Some(remaining) = remaining {
            tokio::time::timeout(remaining, race).await
        } else {
            Ok(race.await)
        } {
            Ok(value) => value,
            Err(_) => Err(ResultValue {
                failed: true,
                deadline_exceeded: true,
                message: "TCP connect deadline exceeded".to_owned(),
                ..ResultValue::default()
            }),
        };
        tasks.abort_all();
        while tasks.join_next().await.is_some() {}
        result
    }) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let (stream, peer) = match outcome {
        Ok(value) => value,
        Err(error) => return error,
    };
    let stream = match stream.into_std() {
        Ok(value) => value,
        Err(error) => return io_error("TCP connect", &error),
    };
    if let Err(error) = stream.set_nonblocking(false) {
        return io_error("TCP connect configuration", &error);
    }
    if let Err(error) = configure_connected_tcp(&stream, no_delay, ttl) {
        return error;
    }
    ResultValue {
        text: peer.to_string(),
        capability: Some(Capability(Arc::new(CapabilityInner::Tcp(TcpState::new(
            stream,
        ))))),
        ..ResultValue::default()
    }
}
pub fn tcp_accept(
    listener: &Capability,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let CapabilityInner::Listener(state) = listener.0.as_ref() else {
        return ResultValue::error("capability is not a TCP listener");
    };
    if let Err(error) = operation_deadline(deadline_nanos, "TCP accept deadline") {
        return error;
    }
    let settings = state.accepted;
    match network_block_on(readiness_operation(
        async move {
            let listener = match state.readiness() {
                Ok(listener) => listener,
                Err(error) => return error,
            };
            match listener.accept().await {
                Ok((stream, peer)) => {
                    let stream = match stream.into_std() {
                        Ok(stream) => stream,
                        Err(error) => return io_error("TCP accept", &error),
                    };
                    if let Err(error) = stream.set_nonblocking(false) {
                        return io_error("TCP accept configuration", &error);
                    }
                    if let Err(error) = settings.apply_std(&stream) {
                        return error;
                    }
                    ResultValue {
                        text: peer.to_string(),
                        capability: Some(Capability(Arc::new(CapabilityInner::Tcp(
                            TcpState::new(stream),
                        )))),
                        ..ResultValue::default()
                    }
                }
                Err(error) => io_error("TCP accept", &error),
            }
        },
        deadline_nanos,
        "TCP accept deadline exceeded",
        cancellation,
    )) {
        Ok(result) => result,
        Err(error) => error,
    }
}
pub fn tcp_read(
    stream: &Capability,
    limit: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let size = match count(limit, "read limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not a TCP stream");
    };
    if size == 0 {
        if !stream.is_open() {
            return ResultValue::error("stream is closed");
        }
        return ResultValue::default();
    }
    match network_block_on(readiness_operation(
        async {
            let stream = match stream.readiness() {
                Ok(value) => value,
                Err(error) => return error,
            };
            let mut data = vec![0; size];
            match networking_state::read(&stream, &mut data).await {
                Ok(count) => {
                    data.truncate(count);
                    ResultValue {
                        data,
                        number: count as i128,
                        flag: count == 0,
                        ..ResultValue::default()
                    }
                }
                Err(error) => io_error("TCP read", &error),
            }
        },
        deadline_nanos,
        "TCP read deadline exceeded",
        cancellation,
    )) {
        Ok(result) => result,
        Err(error) => error,
    }
}
pub fn tcp_write(
    stream: &Capability,
    data: &[u8],
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not a TCP stream");
    };
    match network_block_on(readiness_operation(
        async {
            let stream = match stream.readiness() {
                Ok(value) => value,
                Err(error) => return error,
            };
            match networking_state::write(&stream, data).await {
                Ok(count) => ResultValue {
                    number: count as i128,
                    ..ResultValue::default()
                },
                Err(error) => io_error("TCP write", &error),
            }
        },
        deadline_nanos,
        "TCP write deadline exceeded",
        cancellation,
    )) {
        Ok(result) => result,
        Err(error) => error,
    }
}
pub fn tcp_shutdown(stream: &Capability, direction: &str) -> ResultValue {
    let direction = match direction {
        "read" => Shutdown::Read,
        "write" => Shutdown::Write,
        "both" => Shutdown::Both,
        _ => return ResultValue::error("shutdown direction must be read, write, or both"),
    };
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not a TCP stream");
    };
    match stream.shutdown(direction) {
        Ok(()) => ResultValue::default(),
        Err(error) => io_error("TCP shutdown", &error),
    }
}

pub fn tcp_configure(stream: &Capability, no_delay: bool, ttl: i128) -> ResultValue {
    let ttl = match tcp_ttl(ttl) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return ResultValue::error("capability is not a TCP stream");
    };
    stream
        .configure(TcpSettings { no_delay, ttl })
        .map_or_else(|error| error, |()| ResultValue::default())
}

pub fn udp_bind(address: &str) -> ResultValue {
    match UdpSocket::bind(address) {
        Ok(socket) => {
            let text = socket
                .local_addr()
                .map_or_else(|_| String::new(), |value| value.to_string());
            ResultValue {
                text,
                capability: Some(Capability(Arc::new(CapabilityInner::Udp(UdpState::new(
                    socket,
                ))))),
                ..ResultValue::default()
            }
        }
        Err(error) => ResultValue::error(format!("UDP bind failed: {error}")),
    }
}

pub fn udp_configure(socket: &Capability, broadcast: bool, ttl: i128) -> ResultValue {
    let Ok(ttl) = u32::try_from(ttl) else {
        return ResultValue::error("UDP TTL must be in 0..=4294967295");
    };
    let CapabilityInner::Udp(socket) = socket.0.as_ref() else {
        return ResultValue::error("capability is not a UDP socket");
    };
    let guard = socket.lock();
    let Some(socket) = guard.as_ref() else {
        return ResultValue::error("socket is closed");
    };
    if let Err(error) = socket.set_broadcast(broadcast) {
        return ResultValue::error(format!("cannot set UDP broadcast option: {error}"));
    }
    match socket.set_ttl(ttl) {
        Ok(()) => ResultValue::default(),
        Err(error) => ResultValue::error(format!("cannot set UDP TTL: {error}")),
    }
}
pub fn udp_send_to(
    socket: &Capability,
    data: &[u8],
    address: &str,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    match network_block_on(udp_send_to_async(
        socket,
        data,
        address,
        deadline_nanos,
        cancellation,
    )) {
        Ok(result) => result,
        Err(error) => error,
    }
}
pub fn udp_receive_from(
    socket: &Capability,
    limit: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let size = match count(limit, "datagram limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let CapabilityInner::Udp(socket) = socket.0.as_ref() else {
        return ResultValue::error("capability is not a UDP socket");
    };
    match network_block_on(readiness_operation(
        async {
            let socket = match socket.readiness() {
                Ok(socket) => socket,
                Err(error) => return error,
            };
            let mut data = vec![0; size.saturating_add(1)];
            loop {
                match socket.try_recv_from(&mut data) {
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        if let Err(error) = socket.readable().await {
                            return io_error("UDP receive", &error);
                        }
                    }
                    result => {
                        return match result {
                            Ok((count, peer)) => {
                                let truncated = count > size;
                                data.truncate(count.min(size));
                                ResultValue {
                                    data,
                                    text: peer.to_string(),
                                    number: count.min(size) as i128,
                                    truncated,
                                    ..ResultValue::default()
                                }
                            }
                            Err(error) => io_error("UDP receive", &error),
                        };
                    }
                }
            }
        },
        deadline_nanos,
        "UDP receive deadline exceeded",
        cancellation,
    )) {
        Ok(result) => result,
        Err(error) => error,
    }
}

fn readiness_tcp_stream(stream: &Capability) -> Result<Arc<tokio::net::TcpStream>, ResultValue> {
    let CapabilityInner::Tcp(stream) = stream.0.as_ref() else {
        return Err(ResultValue::error("capability is not a TCP stream"));
    };
    stream.readiness()
}

fn readiness_listener(listener: &Capability) -> Result<Arc<tokio::net::TcpListener>, ResultValue> {
    let CapabilityInner::Listener(listener) = listener.0.as_ref() else {
        return Err(ResultValue::error("capability is not a TCP listener"));
    };
    listener.readiness()
}

fn readiness_udp_socket(socket: &Capability) -> Result<Arc<tokio::net::UdpSocket>, ResultValue> {
    let CapabilityInner::Udp(socket) = socket.0.as_ref() else {
        return Err(ResultValue::error("capability is not a UDP socket"));
    };
    socket.readiness()
}

pub async fn tcp_connect_async(
    address: &str,
    no_delay: bool,
    ttl: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let Ok(address) = address.parse::<SocketAddr>() else {
        return ResultValue::error("invalid socket address");
    };
    let ttl = match tcp_ttl(ttl) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let settings = TcpSettings { no_delay, ttl };
    readiness_operation(
        async move {
            match tokio::net::TcpStream::connect(address).await {
                Ok(stream) => {
                    if let Err(error) = settings.apply_async(&stream) {
                        return error;
                    }
                    ResultValue {
                        capability: Some(Capability(Arc::new(CapabilityInner::Tcp(
                            TcpState::from_async(stream),
                        )))),
                        ..ResultValue::default()
                    }
                }
                Err(error) => io_error("TCP connect", &error),
            }
        },
        deadline_nanos,
        "TCP connect deadline exceeded",
        cancellation,
    )
    .await
}

pub async fn tcp_connect_host_async(
    host: &str,
    port: i128,
    no_delay: bool,
    ttl: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let Ok(port) = u16::try_from(port) else {
        return ResultValue::error("port must be in 0..=65535");
    };
    let ttl = match tcp_ttl(ttl) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let settings = TcpSettings { no_delay, ttl };
    if timeout(deadline_nanos).is_err() {
        return ResultValue::error("deadline must be positive");
    }
    let host = host.to_owned();
    let cancellation_for_dns = cancellation.cloned();
    let resolved = tokio::task::spawn_blocking(move || {
        dns_lookup(
            &host,
            i128::from(port),
            deadline_nanos,
            cancellation_for_dns.as_ref(),
        )
    })
    .await
    .expect("delegated DNS lookup must not panic");
    if resolved.failed {
        return resolved;
    }
    let candidates = resolved
        .entries
        .iter()
        .filter_map(|entry| entry.parse::<SocketAddr>().ok())
        .collect::<Vec<_>>();
    if candidates.is_empty() {
        return ResultValue::error("DNS lookup returned no usable addresses");
    }
    readiness_operation(
        async move {
            let mut tasks = tokio::task::JoinSet::new();
            for (index, address) in candidates.into_iter().enumerate() {
                tasks.spawn(async move {
                    tokio::time::sleep(Duration::from_millis((index as u64).saturating_mul(250)))
                        .await;
                    tokio::net::TcpStream::connect(address)
                        .await
                        .ok()
                        .map(|stream| (stream, address))
                });
            }
            while let Some(joined) = tasks.join_next().await {
                if let Ok(Some((stream, peer))) = joined {
                    if let Err(error) = settings.apply_async(&stream) {
                        tasks.abort_all();
                        return error;
                    }
                    tasks.abort_all();
                    return ResultValue {
                        text: peer.to_string(),
                        capability: Some(Capability(Arc::new(CapabilityInner::Tcp(
                            TcpState::from_async(stream),
                        )))),
                        ..ResultValue::default()
                    };
                }
            }
            ResultValue::error("all TCP connection candidates failed")
        },
        deadline_nanos,
        "TCP connect deadline exceeded",
        cancellation,
    )
    .await
}

pub async fn tcp_accept_async(
    listener: &Capability,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let CapabilityInner::Listener(state) = listener.0.as_ref() else {
        return ResultValue::error("capability is not a TCP listener");
    };
    let settings = state.accepted;
    let listener = match readiness_listener(listener) {
        Ok(value) => value,
        Err(error) => return error,
    };
    readiness_operation(
        async move {
            match listener.accept().await {
                Ok((stream, peer)) => {
                    if let Err(error) = settings.apply_async(&stream) {
                        return error;
                    }
                    ResultValue {
                        text: peer.to_string(),
                        capability: Some(Capability(Arc::new(CapabilityInner::Tcp(
                            TcpState::from_async(stream),
                        )))),
                        ..ResultValue::default()
                    }
                }
                Err(error) => io_error("TCP accept", &error),
            }
        },
        deadline_nanos,
        "TCP accept deadline exceeded",
        cancellation,
    )
    .await
}

pub async fn tcp_read_async(
    stream: &Capability,
    limit: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let size = match count(limit, "read limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let stream = match readiness_tcp_stream(stream) {
        Ok(value) => value,
        Err(error) => return error,
    };
    if size == 0 {
        return ResultValue::default();
    }
    readiness_operation(
        async move {
            let mut data = vec![0; size];
            match networking_state::read(&stream, &mut data).await {
                Ok(count) => {
                    data.truncate(count);
                    ResultValue {
                        data,
                        number: count as i128,
                        flag: count == 0,
                        ..ResultValue::default()
                    }
                }
                Err(error) => io_error("TCP read", &error),
            }
        },
        deadline_nanos,
        "TCP read deadline exceeded",
        cancellation,
    )
    .await
}

pub async fn tcp_write_async(
    stream: &Capability,
    data: &[u8],
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let stream = match readiness_tcp_stream(stream) {
        Ok(value) => value,
        Err(error) => return error,
    };
    readiness_operation(
        async move {
            match networking_state::write(&stream, data).await {
                Ok(count) => ResultValue {
                    number: count as i128,
                    ..ResultValue::default()
                },
                Err(error) => io_error("TCP write", &error),
            }
        },
        deadline_nanos,
        "TCP write deadline exceeded",
        cancellation,
    )
    .await
}

pub async fn udp_send_to_async(
    socket: &Capability,
    data: &[u8],
    address: &str,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let socket = match readiness_udp_socket(socket) {
        Ok(value) => value,
        Err(error) => return error,
    };
    readiness_operation(
        async move {
            match socket.send_to(data, address).await {
                Ok(count) => ResultValue {
                    number: count as i128,
                    ..ResultValue::default()
                },
                Err(error) => io_error("UDP send", &error),
            }
        },
        deadline_nanos,
        "UDP send deadline exceeded",
        cancellation,
    )
    .await
}

pub async fn udp_receive_from_async(
    socket: &Capability,
    limit: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    let size = match count(limit, "datagram limit") {
        Ok(value) => value,
        Err(error) => return error,
    };
    let socket = match readiness_udp_socket(socket) {
        Ok(value) => value,
        Err(error) => return error,
    };
    readiness_operation(
        async move {
            let mut data = vec![0; size.saturating_add(1)];
            match socket.recv_from(&mut data).await {
                Ok((count, peer)) => {
                    let truncated = count > size;
                    data.truncate(count.min(size));
                    ResultValue {
                        data,
                        text: peer.to_string(),
                        number: count.min(size) as i128,
                        truncated,
                        ..ResultValue::default()
                    }
                }
                Err(error) => io_error("UDP receive", &error),
            }
        },
        deadline_nanos,
        "UDP receive deadline exceeded",
        cancellation,
    )
    .await
}

pub(super) fn ordered_socket_candidates(
    addresses: impl IntoIterator<Item = IpAddr>,
    port: u16,
) -> Vec<String> {
    let mut addresses = addresses.into_iter().collect::<Vec<_>>();
    addresses.sort_unstable();
    addresses.dedup();
    addresses
        .into_iter()
        .map(|ip| SocketAddr::new(ip, port).to_string())
        .collect()
}

pub fn dns_lookup(
    host: &str,
    port: i128,
    deadline_nanos: i128,
    cancellation: Option<&Capability>,
) -> ResultValue {
    if let Some(error) = cancellation_error(cancellation) {
        return error;
    }
    let duration = match timeout(deadline_nanos) {
        Ok(value) => value,
        Err(error) => return error,
    };
    let Ok(port) = u16::try_from(port) else {
        return ResultValue::error("port must be in 0..=65535");
    };
    let resolver = match dns_resolver() {
        Ok(value) => value,
        Err(error) => return error,
    };
    let lookup = match network_block_on(async {
        let lookup = async {
            if let Some(duration) = duration {
                tokio::time::timeout(duration, resolver.lookup_ip(host)).await
            } else {
                Ok(resolver.lookup_ip(host).await)
            }
        };
        if let Some(cancellation) = cancellation {
            tokio::select! {
                result = lookup => Some(result),
                _ = cancellation_wait(cancellation) => None,
            }
        } else {
            Some(lookup.await)
        }
    }) {
        Ok(value) => value,
        Err(error) => return error,
    };
    match lookup {
        None => ResultValue::error("operation cancelled"),
        Some(Ok(Ok(values))) => {
            let ttl = values
                .valid_until()
                .saturating_duration_since(std::time::Instant::now())
                .as_secs();
            ResultValue {
                entries: ordered_socket_candidates(values.iter(), port),
                number: i128::from(ttl),
                flag: true,
                ..ResultValue::default()
            }
        }
        Some(Ok(Err(error))) => ResultValue::error(format!("DNS lookup failed: {error}")),
        Some(Err(_)) => ResultValue {
            failed: true,
            deadline_exceeded: true,
            message: "DNS lookup deadline exceeded".to_owned(),
            ..ResultValue::default()
        },
    }
}
