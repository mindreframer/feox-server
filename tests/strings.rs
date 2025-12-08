mod common;

use common::test_data::*;
use common::*;
use redis::Commands;

#[test]
fn test_get_set_basic() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("test");
    let value = "hello";

    let _: () = conn.set(&key, value).unwrap();
    let result: String = conn.get(&key).unwrap();
    assert_eq!(result, value);
}

#[test]
fn test_get_nonexistent() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("nonexistent");
    let result: Option<String> = conn.get(&key).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_set_overwrite() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("overwrite");

    let _: () = conn.set(&key, "first").unwrap();
    let _: () = conn.set(&key, "second").unwrap();
    let result: String = conn.get(&key).unwrap();
    assert_eq!(result, "second");
}

#[test]
fn test_set_with_ex() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("expire");
    let _: () = redis::cmd("SET")
        .arg(&key)
        .arg("value")
        .arg("EX")
        .arg(1)
        .query(&mut conn)
        .unwrap();

    let result: String = conn.get(&key).unwrap();
    assert_eq!(result, "value");

    std::thread::sleep(std::time::Duration::from_secs(2));

    let result: Option<String> = conn.get(&key).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_set_with_px() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("pexpire");
    let _: () = redis::cmd("SET")
        .arg(&key)
        .arg("value")
        .arg("PX")
        .arg(100)
        .query(&mut conn)
        .unwrap();

    let result: String = conn.get(&key).unwrap();
    assert_eq!(result, "value");

    std::thread::sleep(std::time::Duration::from_millis(150));

    let result: Option<String> = conn.get(&key).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_mget_mset() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key1 = unique_key("mtest1");
    let key2 = unique_key("mtest2");
    let key3 = unique_key("mtest3");

    let _: () = redis::cmd("MSET")
        .arg(&key1)
        .arg("value1")
        .arg(&key2)
        .arg("value2")
        .arg(&key3)
        .arg("value3")
        .query(&mut conn)
        .unwrap();

    let result: Vec<String> = redis::cmd("MGET")
        .arg(&key1)
        .arg(&key2)
        .arg(&key3)
        .query(&mut conn)
        .unwrap();

    assert_eq!(result, vec!["value1", "value2", "value3"]);
}

#[test]
fn test_mget_mixed_exists() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key1 = unique_key("exists");
    let key2 = unique_key("nonexist");
    let key3 = unique_key("exists2");

    let _: () = conn.set(&key1, "value1").unwrap();
    let _: () = conn.set(&key3, "value3").unwrap();

    let result: Vec<Option<String>> = redis::cmd("MGET")
        .arg(&key1)
        .arg(&key2)
        .arg(&key3)
        .query(&mut conn)
        .unwrap();

    assert_eq!(result[0], Some("value1".to_string()));
    assert_eq!(result[1], None);
    assert_eq!(result[2], Some("value3".to_string()));
}

#[test]
fn test_incr() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("counter");

    let result: i64 = conn.incr(&key, 1).unwrap();
    assert_eq!(result, 1);

    let result: i64 = conn.incr(&key, 1).unwrap();
    assert_eq!(result, 2);

    let result: i64 = conn.incr(&key, 1).unwrap();
    assert_eq!(result, 3);
}

#[test]
fn test_incrby() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("counter");

    let result: i64 = conn.incr(&key, 5).unwrap();
    assert_eq!(result, 5);

    let result: i64 = conn.incr(&key, 10).unwrap();
    assert_eq!(result, 15);
}

#[test]
fn test_decr() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("counter");
    let _: () = conn.set(&key, "10").unwrap();

    let result: i64 = conn.decr(&key, 1).unwrap();
    assert_eq!(result, 9);

    let result: i64 = conn.decr(&key, 1).unwrap();
    assert_eq!(result, 8);
}

#[test]
fn test_decrby() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("counter");
    let _: () = conn.set(&key, "100").unwrap();

    let result: i64 = conn.decr(&key, 20).unwrap();
    assert_eq!(result, 80);

    let result: i64 = conn.decr(&key, 30).unwrap();
    assert_eq!(result, 50);
}

#[test]
fn test_incr_on_non_integer() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("notint");
    let _: () = conn.set(&key, "notanumber").unwrap();

    let result: Result<i64, _> = conn.incr(&key, 1);
    assert!(result.is_err());
}

#[test]
fn test_del_single() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("todelete");
    let _: () = conn.set(&key, "value").unwrap();

    let deleted: i64 = conn.del(&key).unwrap();
    assert_eq!(deleted, 1);

    let result: Option<String> = conn.get(&key).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_del_multiple() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key1 = unique_key("del1");
    let key2 = unique_key("del2");
    let key3 = unique_key("del3");

    let _: () = conn.set(&key1, "v1").unwrap();
    let _: () = conn.set(&key2, "v2").unwrap();
    let _: () = conn.set(&key3, "v3").unwrap();

    let deleted: i64 = redis::cmd("DEL")
        .arg(&key1)
        .arg(&key2)
        .arg(&key3)
        .query(&mut conn)
        .unwrap();
    assert_eq!(deleted, 3);
}

#[test]
fn test_exists() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("exists");
    let result: i64 = conn.exists(&key).unwrap();
    assert_eq!(result, 0);

    let _: () = conn.set(&key, "value").unwrap();
    let result: i64 = conn.exists(&key).unwrap();
    assert_eq!(result, 1);
}

#[test]
fn test_exists_multiple() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key1 = unique_key("ex1");
    let key2 = unique_key("ex2");
    let key3 = unique_key("ex3");

    let _: () = conn.set(&key1, "v1").unwrap();
    let _: () = conn.set(&key3, "v3").unwrap();

    let result: i64 = redis::cmd("EXISTS")
        .arg(&key1)
        .arg(&key2)
        .arg(&key3)
        .query(&mut conn)
        .unwrap();
    assert_eq!(result, 2);
}

#[test]
fn test_cas_success() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("cas");
    let _: () = conn.set(&key, "initial").unwrap();

    let result: String = redis::cmd("CAS")
        .arg(&key)
        .arg("initial")
        .arg("updated")
        .query(&mut conn)
        .unwrap();
    assert_eq!(result, "OK");

    let value: String = conn.get(&key).unwrap();
    assert_eq!(value, "updated");
}

#[test]
fn test_cas_failure() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("cas");
    let _: () = conn.set(&key, "initial").unwrap();

    let result: Result<String, _> = redis::cmd("CAS")
        .arg(&key)
        .arg("wrong")
        .arg("updated")
        .query(&mut conn);

    assert!(result.is_err());

    let value: String = conn.get(&key).unwrap();
    assert_eq!(value, "initial");
}

#[test]
fn test_jsonpatch() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("json");
    let json_value = r#"{"name":"John","age":30}"#;
    let _: () = conn.set(&key, json_value).unwrap();

    let patch = r#"[{"op":"replace","path":"/age","value":31}]"#;
    let result: String = redis::cmd("JSONPATCH")
        .arg(&key)
        .arg(patch)
        .query(&mut conn)
        .unwrap();

    assert!(result.contains("\"age\":31"));
}

#[test]
fn test_echo() {
    let server = TestServer::new();
    let mut conn = server.client();

    let message = "hello world";
    let result: String = redis::cmd("ECHO").arg(message).query(&mut conn).unwrap();
    assert_eq!(result, message);
}

#[test]
fn test_large_value() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("large");
    let large_value = test_value(1024 * 1024);

    let _: () = conn.set(&key, &large_value).unwrap();
    let result: String = conn.get(&key).unwrap();
    assert_eq!(result.len(), large_value.len());
    assert_eq!(result, large_value);
}
