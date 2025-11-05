mod common;

use common::test_data::*;
use common::*;
use std::collections::HashMap;

#[test]
fn test_hset_hget() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let set: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value1")
        .query(&mut conn)
        .unwrap();
    assert_eq!(set, 1);

    let value: String = redis::cmd("HGET")
        .arg(&key)
        .arg("field1")
        .query(&mut conn)
        .unwrap();
    assert_eq!(value, "value1");
}

#[test]
fn test_hset_multiple() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let set: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value1")
        .arg("field2")
        .arg("value2")
        .arg("field3")
        .arg("value3")
        .query(&mut conn)
        .unwrap();
    assert_eq!(set, 3);
}

#[test]
fn test_hset_overwrite() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let set: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value1")
        .query(&mut conn)
        .unwrap();
    assert_eq!(set, 1);

    let set: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value2")
        .query(&mut conn)
        .unwrap();
    assert_eq!(set, 0);

    let value: String = redis::cmd("HGET")
        .arg(&key)
        .arg("field1")
        .query(&mut conn)
        .unwrap();
    assert_eq!(value, "value2");
}

#[test]
fn test_hget_nonexistent_field() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value1")
        .query(&mut conn)
        .unwrap();

    let result: Option<String> = redis::cmd("HGET")
        .arg(&key)
        .arg("nonexistent")
        .query(&mut conn)
        .unwrap();
    assert!(result.is_none());
}

#[test]
fn test_hget_nonexistent_key() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("nonexistent");
    let result: Option<String> = redis::cmd("HGET")
        .arg(&key)
        .arg("field1")
        .query(&mut conn)
        .unwrap();
    assert!(result.is_none());
}

#[test]
fn test_hmget() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value1")
        .arg("field2")
        .arg("value2")
        .arg("field3")
        .arg("value3")
        .query(&mut conn)
        .unwrap();

    let values: Vec<String> = redis::cmd("HMGET")
        .arg(&key)
        .arg("field1")
        .arg("field2")
        .arg("field3")
        .query(&mut conn)
        .unwrap();

    assert_eq!(values, vec!["value1", "value2", "value3"]);
}

#[test]
fn test_hmget_mixed() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value1")
        .arg("field3")
        .arg("value3")
        .query(&mut conn)
        .unwrap();

    let values: Vec<Option<String>> = redis::cmd("HMGET")
        .arg(&key)
        .arg("field1")
        .arg("field2")
        .arg("field3")
        .query(&mut conn)
        .unwrap();

    assert_eq!(values[0], Some("value1".to_string()));
    assert_eq!(values[1], None);
    assert_eq!(values[2], Some("value3".to_string()));
}

#[test]
fn test_hexists() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value1")
        .query(&mut conn)
        .unwrap();

    let exists: bool = redis::cmd("HEXISTS")
        .arg(&key)
        .arg("field1")
        .query(&mut conn)
        .unwrap();
    assert!(exists);

    let exists: bool = redis::cmd("HEXISTS")
        .arg(&key)
        .arg("field2")
        .query(&mut conn)
        .unwrap();
    assert!(!exists);
}

#[test]
fn test_hdel() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value1")
        .arg("field2")
        .arg("value2")
        .query(&mut conn)
        .unwrap();

    let deleted: i64 = redis::cmd("HDEL")
        .arg(&key)
        .arg("field1")
        .query(&mut conn)
        .unwrap();
    assert_eq!(deleted, 1);

    let exists: bool = redis::cmd("HEXISTS")
        .arg(&key)
        .arg("field1")
        .query(&mut conn)
        .unwrap();
    assert!(!exists);

    let exists: bool = redis::cmd("HEXISTS")
        .arg(&key)
        .arg("field2")
        .query(&mut conn)
        .unwrap();
    assert!(exists);
}

#[test]
fn test_hdel_multiple() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value1")
        .arg("field2")
        .arg("value2")
        .arg("field3")
        .arg("value3")
        .query(&mut conn)
        .unwrap();

    let deleted: i64 = redis::cmd("HDEL")
        .arg(&key)
        .arg("field1")
        .arg("field3")
        .query(&mut conn)
        .unwrap();
    assert_eq!(deleted, 2);

    let len: i64 = redis::cmd("HLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(len, 1);
}

#[test]
fn test_hgetall() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value1")
        .arg("field2")
        .arg("value2")
        .query(&mut conn)
        .unwrap();

    let result: HashMap<String, String> = redis::cmd("HGETALL").arg(&key).query(&mut conn).unwrap();

    assert_eq!(result.len(), 2);
    assert_eq!(result.get("field1"), Some(&"value1".to_string()));
    assert_eq!(result.get("field2"), Some(&"value2".to_string()));
}

#[test]
fn test_hgetall_empty() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("emptyhash");
    let result: HashMap<String, String> = redis::cmd("HGETALL").arg(&key).query(&mut conn).unwrap();

    assert_eq!(result.len(), 0);
}

#[test]
fn test_hlen() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let len: i64 = redis::cmd("HLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(len, 0);

    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value1")
        .query(&mut conn)
        .unwrap();

    let len: i64 = redis::cmd("HLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(len, 1);

    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field2")
        .arg("value2")
        .arg("field3")
        .arg("value3")
        .query(&mut conn)
        .unwrap();

    let len: i64 = redis::cmd("HLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(len, 3);
}

#[test]
fn test_hkeys() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value1")
        .arg("field2")
        .arg("value2")
        .arg("field3")
        .arg("value3")
        .query(&mut conn)
        .unwrap();

    let mut keys: Vec<String> = redis::cmd("HKEYS").arg(&key).query(&mut conn).unwrap();
    keys.sort();

    assert_eq!(keys, vec!["field1", "field2", "field3"]);
}

#[test]
fn test_hvals() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("value1")
        .arg("field2")
        .arg("value2")
        .arg("field3")
        .arg("value3")
        .query(&mut conn)
        .unwrap();

    let mut values: Vec<String> = redis::cmd("HVALS").arg(&key).query(&mut conn).unwrap();
    values.sort();

    assert_eq!(values, vec!["value1", "value2", "value3"]);
}

#[test]
fn test_hincrby() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let result: i64 = redis::cmd("HINCRBY")
        .arg(&key)
        .arg("counter")
        .arg(1)
        .query(&mut conn)
        .unwrap();
    assert_eq!(result, 1);

    let result: i64 = redis::cmd("HINCRBY")
        .arg(&key)
        .arg("counter")
        .arg(5)
        .query(&mut conn)
        .unwrap();
    assert_eq!(result, 6);

    let result: i64 = redis::cmd("HINCRBY")
        .arg(&key)
        .arg("counter")
        .arg(-2)
        .query(&mut conn)
        .unwrap();
    assert_eq!(result, 4);
}

#[test]
fn test_hincrby_on_non_integer() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");
    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field1")
        .arg("notanumber")
        .query(&mut conn)
        .unwrap();

    let result: Result<i64, _> = redis::cmd("HINCRBY")
        .arg(&key)
        .arg("field1")
        .arg(1)
        .query(&mut conn);

    assert!(result.is_err());
}

#[test]
fn test_hash_multiple_operations() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hash");

    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("name")
        .arg("John")
        .arg("age")
        .arg("30")
        .arg("city")
        .arg("NYC")
        .query(&mut conn)
        .unwrap();

    let name: String = redis::cmd("HGET")
        .arg(&key)
        .arg("name")
        .query(&mut conn)
        .unwrap();
    assert_eq!(name, "John");

    let _: i64 = redis::cmd("HINCRBY")
        .arg(&key)
        .arg("age")
        .arg(1)
        .query(&mut conn)
        .unwrap();

    let age: String = redis::cmd("HGET")
        .arg(&key)
        .arg("age")
        .query(&mut conn)
        .unwrap();
    assert_eq!(age, "31");

    let _: i64 = redis::cmd("HDEL")
        .arg(&key)
        .arg("city")
        .query(&mut conn)
        .unwrap();

    let len: i64 = redis::cmd("HLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(len, 2);
}
