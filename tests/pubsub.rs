mod common;

use common::test_data::*;
use common::*;
use std::process::{Command, Stdio};
use std::time::Duration;

#[test]
fn test_publish_no_subscribers() {
    let server = TestServer::new();
    let mut conn = server.client();

    let channel = unique_key("channel");
    let count: i64 = redis::cmd("PUBLISH")
        .arg(&channel)
        .arg("message")
        .query(&mut conn)
        .unwrap();

    assert_eq!(count, 0);
}

#[test]
fn test_subscribe_and_publish() {
    let server = TestServer::new();
    let mut pub_conn = server.client();

    let channel = unique_key("channel");
    let channel_clone = channel.clone();

    let url = server.url();
    let handle = std::thread::spawn(move || {
        let client = redis::Client::open(url).unwrap();
        let mut conn = client.get_connection().unwrap();
        let mut pubsub = conn.as_pubsub();

        pubsub.subscribe(&channel_clone).unwrap();

        let msg = pubsub.get_message().unwrap();
        let payload: String = msg.get_payload().unwrap();
        payload
    });

    std::thread::sleep(Duration::from_millis(100));

    let count: i64 = redis::cmd("PUBLISH")
        .arg(&channel)
        .arg("test message")
        .query(&mut pub_conn)
        .unwrap();

    assert_eq!(count, 1);

    let received = handle.join().unwrap();
    assert_eq!(received, "test message");
}

#[test]
fn test_subscribe_multiple_channels() {
    let server = TestServer::new();
    let mut pub_conn = server.client();

    let channel1 = unique_key("ch1");
    let channel2 = unique_key("ch2");
    let ch1_clone = channel1.clone();
    let ch2_clone = channel2.clone();

    let url = server.url();
    let handle = std::thread::spawn(move || {
        let client = redis::Client::open(url).unwrap();
        let mut conn = client.get_connection().unwrap();
        let mut pubsub = conn.as_pubsub();

        pubsub.subscribe(&[&ch1_clone, &ch2_clone]).unwrap();

        let msg1 = pubsub.get_message().unwrap();
        let payload1: String = msg1.get_payload().unwrap();

        let msg2 = pubsub.get_message().unwrap();
        let payload2: String = msg2.get_payload().unwrap();

        (payload1, payload2)
    });

    std::thread::sleep(Duration::from_millis(100));

    let _: i64 = redis::cmd("PUBLISH")
        .arg(&channel1)
        .arg("message1")
        .query(&mut pub_conn)
        .unwrap();

    let _: i64 = redis::cmd("PUBLISH")
        .arg(&channel2)
        .arg("message2")
        .query(&mut pub_conn)
        .unwrap();

    let (msg1, msg2) = handle.join().unwrap();
    assert_eq!(msg1, "message1");
    assert_eq!(msg2, "message2");
}

#[test]
fn test_unsubscribe() {
    let server = TestServer::new();
    let mut pub_conn = server.client();

    let channel = unique_key("channel");
    let channel_clone = channel.clone();

    let url = server.url();
    let handle = std::thread::spawn(move || {
        let client = redis::Client::open(url).unwrap();
        let mut conn = client.get_connection().unwrap();
        let mut pubsub = conn.as_pubsub();

        pubsub.subscribe(&channel_clone).unwrap();
        pubsub.unsubscribe(&channel_clone).unwrap();

        std::thread::sleep(Duration::from_millis(200));
    });

    std::thread::sleep(Duration::from_millis(100));

    let count: i64 = redis::cmd("PUBLISH")
        .arg(&channel)
        .arg("message")
        .query(&mut pub_conn)
        .unwrap();

    assert_eq!(count, 0);

    handle.join().unwrap();
}

#[test]
fn test_psubscribe_pattern() {
    let server = TestServer::new();
    let mut pub_conn = server.client();

    let prefix = unique_key("pattern");
    let pattern = format!("{}:*", prefix);
    let channel1 = format!("{}:ch1", prefix);
    let channel2 = format!("{}:ch2", prefix);

    let pattern_clone = pattern.clone();
    let url = server.url();
    let handle = std::thread::spawn(move || {
        let client = redis::Client::open(url).unwrap();
        let mut conn = client.get_connection().unwrap();
        let mut pubsub = conn.as_pubsub();

        pubsub.psubscribe(&pattern_clone).unwrap();

        let msg1 = pubsub.get_message().unwrap();
        let payload1: String = msg1.get_payload().unwrap();

        let msg2 = pubsub.get_message().unwrap();
        let payload2: String = msg2.get_payload().unwrap();

        (payload1, payload2)
    });

    std::thread::sleep(Duration::from_millis(100));

    let _: i64 = redis::cmd("PUBLISH")
        .arg(&channel1)
        .arg("msg1")
        .query(&mut pub_conn)
        .unwrap();

    let _: i64 = redis::cmd("PUBLISH")
        .arg(&channel2)
        .arg("msg2")
        .query(&mut pub_conn)
        .unwrap();

    let (msg1, msg2) = handle.join().unwrap();
    assert_eq!(msg1, "msg1");
    assert_eq!(msg2, "msg2");
}

#[test]
fn test_pubsub_channels() {
    let server = TestServer::new();
    let mut pub_conn = server.client();

    let channel1 = unique_key("ch1");
    let channel2 = unique_key("ch2");
    let ch1_clone = channel1.clone();
    let ch2_clone = channel2.clone();

    let url = server.url();
    let handle = std::thread::spawn(move || {
        let client = redis::Client::open(url).unwrap();
        let mut conn = client.get_connection().unwrap();
        let mut pubsub = conn.as_pubsub();

        pubsub.subscribe(&[&ch1_clone, &ch2_clone]).unwrap();

        std::thread::sleep(Duration::from_millis(500));
    });

    std::thread::sleep(Duration::from_millis(100));

    let channels: Vec<String> = redis::cmd("PUBSUB")
        .arg("CHANNELS")
        .query(&mut pub_conn)
        .unwrap();

    assert!(channels.contains(&channel1));
    assert!(channels.contains(&channel2));

    handle.join().unwrap();
}

#[test]
fn test_pubsub_numsub() {
    let server = TestServer::new();
    let mut pub_conn = server.client();

    let channel = unique_key("numsub");
    let channel_clone = channel.clone();

    let url = server.url();
    let handle = std::thread::spawn(move || {
        let client = redis::Client::open(url).unwrap();
        let mut conn = client.get_connection().unwrap();
        let mut pubsub = conn.as_pubsub();

        pubsub.subscribe(&channel_clone).unwrap();

        std::thread::sleep(Duration::from_millis(500));
    });

    std::thread::sleep(Duration::from_millis(100));

    let result: Vec<redis::Value> = redis::cmd("PUBSUB")
        .arg("NUMSUB")
        .arg(&channel)
        .query(&mut pub_conn)
        .unwrap();

    assert_eq!(result.len(), 2);
    if let redis::Value::Data(channel_bytes) = &result[0] {
        assert_eq!(String::from_utf8_lossy(channel_bytes), channel);
    }
    if let redis::Value::Int(count) = result[1] {
        assert_eq!(count, 1);
    }

    handle.join().unwrap();
}

#[test]
fn test_pubsub_numpat() {
    let server = TestServer::new();
    let mut pub_conn = server.client();

    let pattern = unique_key("pattern:*");
    let pattern_clone = pattern.clone();

    let url = server.url();
    let handle = std::thread::spawn(move || {
        let client = redis::Client::open(url).unwrap();
        let mut conn = client.get_connection().unwrap();
        let mut pubsub = conn.as_pubsub();

        pubsub.psubscribe(&pattern_clone).unwrap();

        std::thread::sleep(Duration::from_millis(500));
    });

    std::thread::sleep(Duration::from_millis(100));

    let count: i64 = redis::cmd("PUBSUB")
        .arg("NUMPAT")
        .query(&mut pub_conn)
        .unwrap();

    assert!(count >= 1);

    handle.join().unwrap();
}

#[test]
fn test_redis_cli_interactive_subscribe() {
    let server = TestServer::new();
    let port = server.port();

    let output_file = format!("/tmp/redis_cli_test_{}.txt", unique_key("sub"));

    let subscriber = std::thread::spawn(move || {
        Command::new("redis-cli")
            .arg("-p")
            .arg(port.to_string())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                use std::io::Write;
                if let Some(mut stdin) = child.stdin.take() {
                    stdin.write_all(b"SUBSCRIBE testchannel\n")?;
                }
                std::thread::sleep(Duration::from_millis(2000));
                child.kill()?;
                child.wait_with_output()
            })
            .ok()
    });

    std::thread::sleep(Duration::from_millis(200));

    let mut pub_conn = server.client();
    let count: i64 = redis::cmd("PUBLISH")
        .arg("testchannel")
        .arg("hello from test")
        .query(&mut pub_conn)
        .unwrap();

    assert!(count >= 1);

    if let Some(result) = subscriber.join().unwrap() {
        let output = String::from_utf8_lossy(&result.stdout);
        assert!(output.contains("subscribe"));
        assert!(output.contains("testchannel"));
    }

    let _ = std::fs::remove_file(output_file);
}

#[test]
fn test_multiple_subscribers_same_channel() {
    let server = TestServer::new();
    let mut pub_conn = server.client();

    let channel = unique_key("multi");
    let ch1 = channel.clone();
    let ch2 = channel.clone();

    let url1 = server.url();
    let url2 = server.url();

    let handle1 = std::thread::spawn(move || {
        let client = redis::Client::open(url1).unwrap();
        let mut conn = client.get_connection().unwrap();
        let mut pubsub = conn.as_pubsub();
        pubsub.subscribe(&ch1).unwrap();
        let msg = pubsub.get_message().unwrap();
        msg.get_payload::<String>().unwrap()
    });

    let handle2 = std::thread::spawn(move || {
        let client = redis::Client::open(url2).unwrap();
        let mut conn = client.get_connection().unwrap();
        let mut pubsub = conn.as_pubsub();
        pubsub.subscribe(&ch2).unwrap();
        let msg = pubsub.get_message().unwrap();
        msg.get_payload::<String>().unwrap()
    });

    std::thread::sleep(Duration::from_millis(100));

    let count: i64 = redis::cmd("PUBLISH")
        .arg(&channel)
        .arg("broadcast message")
        .query(&mut pub_conn)
        .unwrap();

    assert_eq!(count, 2);

    let msg1 = handle1.join().unwrap();
    let msg2 = handle2.join().unwrap();

    assert_eq!(msg1, "broadcast message");
    assert_eq!(msg2, "broadcast message");
}
