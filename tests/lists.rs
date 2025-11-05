mod common;

use common::test_data::*;
use common::*;

#[test]
fn test_lpush_llen() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("list");
    let len: i64 = redis::cmd("LPUSH")
        .arg(&key)
        .arg("value1")
        .query(&mut conn)
        .unwrap();
    assert_eq!(len, 1);

    let len: i64 = redis::cmd("LPUSH")
        .arg(&key)
        .arg("value2")
        .query(&mut conn)
        .unwrap();
    assert_eq!(len, 2);

    let llen: i64 = redis::cmd("LLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(llen, 2);
}

#[test]
fn test_lpush_multiple() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("list");
    let len: i64 = redis::cmd("LPUSH")
        .arg(&key)
        .arg("v1")
        .arg("v2")
        .arg("v3")
        .query(&mut conn)
        .unwrap();
    assert_eq!(len, 3);
}

#[test]
fn test_rpush_llen() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("list");
    let len: i64 = redis::cmd("RPUSH")
        .arg(&key)
        .arg("value1")
        .query(&mut conn)
        .unwrap();
    assert_eq!(len, 1);

    let len: i64 = redis::cmd("RPUSH")
        .arg(&key)
        .arg("value2")
        .query(&mut conn)
        .unwrap();
    assert_eq!(len, 2);

    let llen: i64 = redis::cmd("LLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(llen, 2);
}

#[test]
fn test_lpush_lrange() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("list");
    let _: i64 = redis::cmd("LPUSH")
        .arg(&key)
        .arg("v1")
        .arg("v2")
        .arg("v3")
        .query(&mut conn)
        .unwrap();

    let result: Vec<String> = redis::cmd("LRANGE")
        .arg(&key)
        .arg(0)
        .arg(-1)
        .query(&mut conn)
        .unwrap();

    assert_eq!(result, vec!["v3", "v2", "v1"]);
}

#[test]
fn test_rpush_lrange() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("list");
    let _: i64 = redis::cmd("RPUSH")
        .arg(&key)
        .arg("v1")
        .arg("v2")
        .arg("v3")
        .query(&mut conn)
        .unwrap();

    let result: Vec<String> = redis::cmd("LRANGE")
        .arg(&key)
        .arg(0)
        .arg(-1)
        .query(&mut conn)
        .unwrap();

    assert_eq!(result, vec!["v1", "v2", "v3"]);
}

#[test]
fn test_lrange_partial() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("list");
    let _: i64 = redis::cmd("RPUSH")
        .arg(&key)
        .arg("v1")
        .arg("v2")
        .arg("v3")
        .arg("v4")
        .arg("v5")
        .query(&mut conn)
        .unwrap();

    let result: Vec<String> = redis::cmd("LRANGE")
        .arg(&key)
        .arg(1)
        .arg(3)
        .query(&mut conn)
        .unwrap();

    assert_eq!(result, vec!["v2", "v3", "v4"]);
}

#[test]
fn test_lpop() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("list");
    let _: i64 = redis::cmd("RPUSH")
        .arg(&key)
        .arg("v1")
        .arg("v2")
        .arg("v3")
        .query(&mut conn)
        .unwrap();

    let value: String = redis::cmd("LPOP").arg(&key).query(&mut conn).unwrap();
    assert_eq!(value, "v1");

    let llen: i64 = redis::cmd("LLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(llen, 2);
}

#[test]
fn test_lpop_count() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("list");
    let _: i64 = redis::cmd("RPUSH")
        .arg(&key)
        .arg("v1")
        .arg("v2")
        .arg("v3")
        .arg("v4")
        .query(&mut conn)
        .unwrap();

    let values: Vec<String> = redis::cmd("LPOP")
        .arg(&key)
        .arg(2)
        .query(&mut conn)
        .unwrap();
    assert_eq!(values, vec!["v1", "v2"]);

    let llen: i64 = redis::cmd("LLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(llen, 2);
}

#[test]
fn test_rpop() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("list");
    let _: i64 = redis::cmd("RPUSH")
        .arg(&key)
        .arg("v1")
        .arg("v2")
        .arg("v3")
        .query(&mut conn)
        .unwrap();

    let value: String = redis::cmd("RPOP").arg(&key).query(&mut conn).unwrap();
    assert_eq!(value, "v3");

    let llen: i64 = redis::cmd("LLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(llen, 2);
}

#[test]
fn test_rpop_count() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("list");
    let _: i64 = redis::cmd("RPUSH")
        .arg(&key)
        .arg("v1")
        .arg("v2")
        .arg("v3")
        .arg("v4")
        .query(&mut conn)
        .unwrap();

    let values: Vec<String> = redis::cmd("RPOP")
        .arg(&key)
        .arg(2)
        .query(&mut conn)
        .unwrap();
    assert_eq!(values, vec!["v4", "v3"]);

    let llen: i64 = redis::cmd("LLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(llen, 2);
}

#[test]
fn test_lpop_empty() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("emptylist");
    let result: Option<String> = redis::cmd("LPOP").arg(&key).query(&mut conn).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_rpop_empty() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("emptylist");
    let result: Option<String> = redis::cmd("RPOP").arg(&key).query(&mut conn).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_lindex() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("list");
    let _: i64 = redis::cmd("RPUSH")
        .arg(&key)
        .arg("v1")
        .arg("v2")
        .arg("v3")
        .query(&mut conn)
        .unwrap();

    let value: String = redis::cmd("LINDEX")
        .arg(&key)
        .arg(0)
        .query(&mut conn)
        .unwrap();
    assert_eq!(value, "v1");

    let value: String = redis::cmd("LINDEX")
        .arg(&key)
        .arg(1)
        .query(&mut conn)
        .unwrap();
    assert_eq!(value, "v2");

    let value: String = redis::cmd("LINDEX")
        .arg(&key)
        .arg(2)
        .query(&mut conn)
        .unwrap();
    assert_eq!(value, "v3");
}

#[test]
fn test_lindex_negative() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("list");
    let _: i64 = redis::cmd("RPUSH")
        .arg(&key)
        .arg("v1")
        .arg("v2")
        .arg("v3")
        .query(&mut conn)
        .unwrap();

    let value: String = redis::cmd("LINDEX")
        .arg(&key)
        .arg(-1)
        .query(&mut conn)
        .unwrap();
    assert_eq!(value, "v3");

    let value: String = redis::cmd("LINDEX")
        .arg(&key)
        .arg(-2)
        .query(&mut conn)
        .unwrap();
    assert_eq!(value, "v2");
}

#[test]
fn test_lindex_out_of_bounds() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("list");
    let _: i64 = redis::cmd("RPUSH")
        .arg(&key)
        .arg("v1")
        .query(&mut conn)
        .unwrap();

    let result: Option<String> = redis::cmd("LINDEX")
        .arg(&key)
        .arg(10)
        .query(&mut conn)
        .unwrap();
    assert!(result.is_none());
}

#[test]
fn test_llen_nonexistent() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("nonexistent");
    let llen: i64 = redis::cmd("LLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(llen, 0);
}

#[test]
fn test_list_fifo_behavior() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("queue");

    let _: i64 = redis::cmd("RPUSH")
        .arg(&key)
        .arg("first")
        .arg("second")
        .arg("third")
        .query(&mut conn)
        .unwrap();

    let val: String = redis::cmd("LPOP").arg(&key).query(&mut conn).unwrap();
    assert_eq!(val, "first");

    let val: String = redis::cmd("LPOP").arg(&key).query(&mut conn).unwrap();
    assert_eq!(val, "second");

    let val: String = redis::cmd("LPOP").arg(&key).query(&mut conn).unwrap();
    assert_eq!(val, "third");
}

#[test]
fn test_list_lifo_behavior() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("stack");

    let _: i64 = redis::cmd("LPUSH")
        .arg(&key)
        .arg("first")
        .arg("second")
        .arg("third")
        .query(&mut conn)
        .unwrap();

    let val: String = redis::cmd("LPOP").arg(&key).query(&mut conn).unwrap();
    assert_eq!(val, "third");

    let val: String = redis::cmd("LPOP").arg(&key).query(&mut conn).unwrap();
    assert_eq!(val, "second");

    let val: String = redis::cmd("LPOP").arg(&key).query(&mut conn).unwrap();
    assert_eq!(val, "first");
}
