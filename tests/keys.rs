mod common;

use common::test_data::*;
use common::*;
use redis::Commands;

#[test]
fn test_expire_ttl() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("expire");
    let _: () = conn.set(&key, "value").unwrap();

    let result: bool = redis::cmd("EXPIRE")
        .arg(&key)
        .arg(10)
        .query(&mut conn)
        .unwrap();
    assert!(result);

    let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut conn).unwrap();
    assert!(ttl > 0 && ttl <= 10);
}

#[test]
fn test_pexpire_pttl() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("pexpire");
    let _: () = conn.set(&key, "value").unwrap();

    let result: bool = redis::cmd("PEXPIRE")
        .arg(&key)
        .arg(10000)
        .query(&mut conn)
        .unwrap();
    assert!(result);

    let pttl: i64 = redis::cmd("PTTL").arg(&key).query(&mut conn).unwrap();
    assert!(pttl > 0 && pttl <= 10000);
}

#[test]
fn test_ttl_nonexistent() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("nonexistent");
    let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut conn).unwrap();
    assert_eq!(ttl, -2);
}

#[test]
fn test_ttl_no_expiry() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("noexpiry");
    let _: () = conn.set(&key, "value").unwrap();

    let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut conn).unwrap();
    assert_eq!(ttl, -1);
}

#[test]
fn test_persist() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("persist");
    let _: () = conn.set(&key, "value").unwrap();

    let _: bool = redis::cmd("EXPIRE")
        .arg(&key)
        .arg(10)
        .query(&mut conn)
        .unwrap();

    let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut conn).unwrap();
    assert!(ttl > 0);

    let result: bool = redis::cmd("PERSIST").arg(&key).query(&mut conn).unwrap();
    assert!(result);

    let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut conn).unwrap();
    assert_eq!(ttl, -1);
}

#[test]
fn test_persist_nonexistent() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("nonexistent");
    let result: bool = redis::cmd("PERSIST").arg(&key).query(&mut conn).unwrap();
    assert!(!result);
}

#[test]
fn test_expire_actually_expires() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("toexpire");
    let _: () = conn.set(&key, "value").unwrap();

    let _: bool = redis::cmd("EXPIRE")
        .arg(&key)
        .arg(1)
        .query(&mut conn)
        .unwrap();

    let value: String = conn.get(&key).unwrap();
    assert_eq!(value, "value");

    std::thread::sleep(std::time::Duration::from_millis(1100));

    let result: Option<String> = conn.get(&key).unwrap();
    assert!(result.is_none());
}

#[test]
fn test_keys_pattern_all() {
    let server = TestServer::new();
    let mut conn = server.client();

    let prefix = unique_key("keystest");
    let key1 = format!("{}_1", prefix);
    let key2 = format!("{}_2", prefix);
    let key3 = format!("{}_3", prefix);

    let _: () = conn.set(&key1, "v1").unwrap();
    let _: () = conn.set(&key2, "v2").unwrap();
    let _: () = conn.set(&key3, "v3").unwrap();

    let pattern = format!("{}_*", prefix);
    let mut keys: Vec<String> = redis::cmd("KEYS").arg(&pattern).query(&mut conn).unwrap();
    keys.sort();

    assert_eq!(keys.len(), 3);
    assert!(keys.contains(&key1));
    assert!(keys.contains(&key2));
    assert!(keys.contains(&key3));
}

#[test]
fn test_keys_pattern_specific() {
    let server = TestServer::new();
    let mut conn = server.client();

    let prefix = unique_key("pattern");
    let key1 = format!("{}_abc", prefix);
    let key2 = format!("{}_def", prefix);
    let key3 = format!("{}_xyz", prefix);

    let _: () = conn.set(&key1, "v1").unwrap();
    let _: () = conn.set(&key2, "v2").unwrap();
    let _: () = conn.set(&key3, "v3").unwrap();

    let pattern = format!("{}_a*", prefix);
    let keys: Vec<String> = redis::cmd("KEYS").arg(&pattern).query(&mut conn).unwrap();

    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0], key1);
}

#[test]
fn test_scan() {
    let server = TestServer::new();
    let mut conn = server.client();

    let prefix = unique_key("scan");
    for i in 0..10 {
        let key = format!("{}_{}", prefix, i);
        let _: () = conn.set(&key, format!("value{}", i)).unwrap();
    }

    let result: (String, Vec<String>) = redis::cmd("SCAN")
        .arg("0")
        .arg("MATCH")
        .arg(format!("{}_*", prefix))
        .arg("COUNT")
        .arg(100)
        .query(&mut conn)
        .unwrap();

    let (cursor, keys) = result;
    assert_eq!(cursor, "0");
    assert_eq!(keys.len(), 10);
}

#[test]
#[ignore = "FLUSHDB not implemented in memory-only mode"]
fn test_flushdb() {
    let server = TestServer::new();
    let mut conn = server.client();

    let prefix = unique_key("flush");
    for i in 0..5 {
        let key = format!("{}_{}", prefix, i);
        let _: () = conn.set(&key, format!("value{}", i)).unwrap();
    }

    let pattern = format!("{}_*", prefix);
    let keys: Vec<String> = redis::cmd("KEYS").arg(&pattern).query(&mut conn).unwrap();
    assert_eq!(keys.len(), 5);

    let _: () = redis::cmd("FLUSHDB").query(&mut conn).unwrap();

    let keys: Vec<String> = redis::cmd("KEYS").arg(&pattern).query(&mut conn).unwrap();
    assert_eq!(keys.len(), 0);
}

#[test]
fn test_del_with_ttl() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("delttl");
    let _: () = conn.set(&key, "value").unwrap();

    let _: bool = redis::cmd("EXPIRE")
        .arg(&key)
        .arg(10)
        .query(&mut conn)
        .unwrap();

    let deleted: i64 = conn.del(&key).unwrap();
    assert_eq!(deleted, 1);

    let exists: i64 = conn.exists(&key).unwrap();
    assert_eq!(exists, 0);
}

#[test]
fn test_overwrite_removes_ttl() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("overwrite");
    let _: () = conn.set(&key, "value1").unwrap();

    let _: bool = redis::cmd("EXPIRE")
        .arg(&key)
        .arg(10)
        .query(&mut conn)
        .unwrap();

    let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut conn).unwrap();
    assert!(ttl > 0);

    let _: () = conn.set(&key, "value2").unwrap();

    let ttl: i64 = redis::cmd("TTL").arg(&key).query(&mut conn).unwrap();
    assert_eq!(ttl, -1);
}

#[test]
#[ignore = "EXPIRE on list keys not yet implemented"]
fn test_expire_on_list() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("listexpire");
    let _: i64 = redis::cmd("LPUSH")
        .arg(&key)
        .arg("value")
        .query(&mut conn)
        .unwrap();

    let _: bool = redis::cmd("EXPIRE")
        .arg(&key)
        .arg(1)
        .query(&mut conn)
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1100));

    let llen: i64 = redis::cmd("LLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(llen, 0);
}

#[test]
#[ignore = "EXPIRE on hash keys not yet implemented"]
fn test_expire_on_hash() {
    let server = TestServer::new();
    let mut conn = server.client();

    let key = unique_key("hashexpire");
    let _: i64 = redis::cmd("HSET")
        .arg(&key)
        .arg("field")
        .arg("value")
        .query(&mut conn)
        .unwrap();

    let _: bool = redis::cmd("EXPIRE")
        .arg(&key)
        .arg(1)
        .query(&mut conn)
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1100));

    let hlen: i64 = redis::cmd("HLEN").arg(&key).query(&mut conn).unwrap();
    assert_eq!(hlen, 0);
}
