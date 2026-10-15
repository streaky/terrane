use super::{Capability, CapabilityInner, ResultValue};
use std::io;

/// The owned native TCP stream used by ecosystem integrations.
pub type NativeTcpStream = tokio::net::TcpStream;

/// The owned native TCP listener used by ecosystem integrations.
pub type NativeTcpListener = tokio::net::TcpListener;

fn transfer_error(error: ResultValue) -> io::Error {
    io::Error::other(error.message)
}

/// Detaches a Terrane TCP listener and transfers its registered transport.
///
/// # Errors
///
/// Rejects non-listener, closed, busy, or off-runtime capabilities, and IO registration errors.
pub fn into_native_tcp_listener(capability: &Capability) -> io::Result<NativeTcpListener> {
    let CapabilityInner::Listener(listener) = capability.0.as_ref() else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "capability is not a TCP listener",
        ));
    };
    listener.take_async().map_err(transfer_error)
}

/// Detaches a Terrane TCP stream and transfers its registered transport.
///
/// # Errors
///
/// Rejects non-stream, closed, busy, or off-runtime capabilities, and IO registration errors.
pub fn into_native_tcp_stream(capability: &Capability) -> io::Result<NativeTcpStream> {
    let CapabilityInner::Tcp(stream) = capability.0.as_ref() else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "capability is not a TCP stream",
        ));
    };
    stream.take_native_async().map_err(transfer_error)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        close,
        networking_state::{ListenerState, TcpSettings, TcpState},
    };
    use std::sync::Arc;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    fn tcp_capability(stream: tokio::net::TcpStream) -> Capability {
        Capability(Arc::new(CapabilityInner::Tcp(TcpState::from_async(stream))))
    }

    #[tokio::test]
    async fn transferred_stream_survives_source_close_and_half_closes() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let client = tokio::net::TcpStream::connect(address).await.unwrap();
        let (server, _) = listener.accept().await.unwrap();
        let source = tcp_capability(client);
        let mut client = into_native_tcp_stream(&source).unwrap();
        assert!(!close(&source).failed);
        client.write_all(b"payload").await.unwrap();
        client.shutdown().await.unwrap();
        let mut received = Vec::new();
        let mut server = server;
        server.read_to_end(&mut received).await.unwrap();
        assert_eq!(received, b"payload");
        server.write_all(b"reply").await.unwrap();
        server.shutdown().await.unwrap();
        let mut reply = Vec::new();
        client.read_to_end(&mut reply).await.unwrap();
        assert_eq!(reply, b"reply");
    }

    #[tokio::test]
    async fn transferred_listener_preserves_accepted_settings() {
        let bound = crate::tcp_bind("127.0.0.1:0", 128, false, true, 23);
        assert!(!bound.failed, "{}", bound.message);
        let address = bound.text;
        let capability = bound.capability.unwrap();
        let _client = tokio::net::TcpStream::connect(address).await.unwrap();
        let listener = into_native_tcp_listener(&capability).unwrap();
        let (stream, _) = listener.accept().await.unwrap();
        assert!(stream.nodelay().unwrap());
        assert_eq!(stream.ttl().unwrap(), 23);
        assert!(!close(&capability).failed);
        assert!(listener.local_addr().is_ok());
    }

    #[tokio::test]
    async fn busy_stream_transfer_does_not_lose_source() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client = tokio::net::TcpStream::connect(listener.local_addr().unwrap())
            .await
            .unwrap();
        let _ = listener.accept().await.unwrap();
        let source = tcp_capability(client);
        let CapabilityInner::Tcp(state) = source.0.as_ref() else {
            unreachable!()
        };
        let retained = state
            .readiness()
            .unwrap_or_else(|error| panic!("{}", error.message));
        assert_eq!(
            into_native_tcp_stream(&source).err().unwrap().kind(),
            io::ErrorKind::Other
        );
        assert!(state.is_open());
        drop(retained);
        assert!(into_native_tcp_stream(&source).is_ok());
    }


    #[tokio::test]
    async fn invalid_listener_transfer_is_rejected() {
        let error = into_native_tcp_listener(&Capability::default())
            .err()
            .unwrap();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
    }

    #[tokio::test]
    async fn closed_stream_transfer_is_rejected() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client = tokio::net::TcpStream::connect(listener.local_addr().unwrap())
            .await
            .unwrap();
        let _ = listener.accept().await.unwrap();
        let source = tcp_capability(client);
        assert!(!close(&source).failed);
        assert!(into_native_tcp_stream(&source).is_err());
    }
    #[tokio::test]
    async fn busy_listener_transfer_does_not_lose_source() {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let capability = Capability(Arc::new(CapabilityInner::Listener(ListenerState::new(
            listener,
            TcpSettings {
                no_delay: false,
                ttl: None,
            },
        ))));
        let CapabilityInner::Listener(state) = capability.0.as_ref() else {
            unreachable!()
        };
        let retained = state
            .readiness()
            .unwrap_or_else(|error| panic!("{}", error.message));
        assert_eq!(
            into_native_tcp_listener(&capability).err().unwrap().kind(),
            io::ErrorKind::Other
        );
        assert!(state.read().is_some());
        drop(retained);
        assert!(into_native_tcp_listener(&capability).is_ok());
    }
}
