use super::*;
use std::future::Future;
use std::task::Poll;

#[test]
fn sync_udp_after_async_honors_deadline() {
    let origin = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let bound = udp_bind("127.0.0.1:0");
    let address = bound.text;
    let socket = bound.capability.unwrap();
    let sender = UdpSocket::bind("127.0.0.1:0").unwrap();
    sender.send_to(b"first", &address).unwrap();
    let first = origin.block_on(udp_receive_from_async(&socket, 16, 1_000_000_000, None));
    assert_eq!(first.data, b"first");
    let timeout = udp_receive_from(&socket, 16, 10_000_000, None);
    assert!(timeout.failed && timeout.deadline_exceeded);
    sender.send_to(b"second", &address).unwrap();
    let second = udp_receive_from(&socket, 16, 1_000_000_000, None);
    assert!(!second.failed, "{}", second.message);
    assert_eq!(second.data, b"second");
    assert!(!close(&socket).failed);
}

#[test]
fn retained_udp_wakes_peer_and_reports_shutdown() {
    let origin = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let other = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let bound = udp_bind("127.0.0.1:0");
    let address = bound.text;
    let socket = bound.capability.unwrap();
    let sender = UdpSocket::bind("127.0.0.1:0").unwrap();
    sender.send_to(b"initial", &address).unwrap();
    assert_eq!(
        origin
            .block_on(udp_receive_from_async(&socket, 16, 1_000_000_000, None))
            .data,
        b"initial"
    );
    let received = other.block_on(async {
        let mut read = std::pin::pin!(udp_receive_from_async(&socket, 16, 1_000_000_000, None));
        let mut sent = false;
        std::future::poll_fn(|cx| match read.as_mut().poll(cx) {
            Poll::Pending => {
                if !sent {
                    sent = true;
                    sender.send_to(b"wake", &address).unwrap();
                }
                Poll::Pending
            }
            Poll::Ready(result) => Poll::Ready(result),
        })
        .await
    });
    assert!(!received.failed, "{}", received.message);
    assert_eq!(received.data, b"wake");
    drop(origin);
    let stopped = other.block_on(udp_receive_from_async(&socket, 16, 1_000_000_000, None));
    assert!(
        stopped.failed && !stopped.deadline_exceeded,
        "{}",
        stopped.message
    );
    assert!(!close(&socket).failed);
}

#[test]
fn sync_tcp_needs_explicit_blocking_delegation() {
    let bound = tcp_bind("127.0.0.1:0", 16, false, false, -1);
    let listener = bound.capability.unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let rejected = runtime.block_on(async { tcp_accept(&listener, 1_000_000_000, None) });
    assert!(rejected.failed && !rejected.deadline_exceeded);
    assert!(rejected.capability.is_none());
    assert!(!close(&listener).failed);
}

#[tokio::test(flavor = "current_thread")]
async fn sync_buffer_callback_preserves_async_progress() {
    let buffer = buffer_create(4).capability.unwrap();
    let result = buffer_read_with(&buffer, |storage| {
        storage[..2].copy_from_slice(b"ab");
        Ok(2)
    });
    assert!(!result.failed && result.number == 2);
    assert_eq!(platform_buffer_completed(&buffer).number, 2);
    assert_eq!(buffer_filled_bytes(&buffer).data, b"ab");
    let CapabilityInner::Buffer(state) = buffer.0.as_ref() else {
        unreachable!()
    };
    let guard = state.lock().await;
    let busy = buffer_read_with(&buffer, |_| panic!("busy callback must not run"));
    assert!(busy.failed);
    drop(guard);
    assert_eq!(buffer_filled_bytes(&buffer).data, b"ab");
    assert!(!close(&buffer).failed);
}
