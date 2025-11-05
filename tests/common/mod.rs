use redis::{Client, Connection};
use std::process::{Child, Command};
use std::sync::atomic::{AtomicU16, Ordering};
use std::thread;
use std::time::Duration;

pub mod test_data;

static PORT_COUNTER: AtomicU16 = AtomicU16::new(20000);

pub struct TestServer {
    process: Option<Child>,
    #[allow(dead_code)]
    port: u16,
    client: Client,
}

impl TestServer {
    pub fn new() -> Self {
        Self::with_threads(4)
    }

    pub fn with_threads(threads: usize) -> Self {
        let port = PORT_COUNTER.fetch_add(1, Ordering::SeqCst);

        let server_path = env!("CARGO_BIN_EXE_feox-server");

        let log_file = std::fs::File::create(format!("/tmp/feox_server_{}.log", port))
            .expect("Failed to create log file");
        let log_file_clone = log_file.try_clone().expect("Failed to clone log file");

        let mut process = Command::new(server_path)
            .arg("--port")
            .arg(port.to_string())
            .arg("--threads")
            .arg(threads.to_string())
            .stdout(log_file)
            .stderr(log_file_clone)
            .spawn()
            .expect("Failed to start feox-server");

        thread::sleep(Duration::from_millis(100));

        if let Ok(Some(status)) = process.try_wait() {
            panic!("Server exited immediately with status: {}", status);
        }

        let client = Client::open(format!("redis://127.0.0.1:{}", port))
            .expect("Failed to create redis client");

        for _ in 0..50 {
            if client.get_connection().is_ok() {
                return Self {
                    process: Some(process),
                    port,
                    client,
                };
            }
            thread::sleep(Duration::from_millis(50));
        }

        panic!("Server failed to start within timeout");
    }

    pub fn client(&self) -> Connection {
        self.client
            .get_connection()
            .expect("Failed to get connection")
    }

    #[allow(dead_code)]
    pub fn port(&self) -> u16 {
        self.port
    }

    #[allow(dead_code)]
    pub fn url(&self) -> String {
        format!("redis://127.0.0.1:{}", self.port)
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        if let Some(mut process) = self.process.take() {
            let _ = process.kill();
            let _ = process.wait();
        }
    }
}
