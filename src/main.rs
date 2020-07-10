mod thread_handler;

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;

const REQUEST_AMOUNT: usize = 100;
const THREAD_AMOUNT: usize = 8;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    let pool = thread_handler::ThreadPool::new(THREAD_AMOUNT);

    for stream in listener.incoming().take(REQUEST_AMOUNT) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_client(stream);
        });
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = vec![0; 4096];
    let bytes_read = stream.read(&mut buffer).unwrap();
    println!("bytes read: {}", bytes_read);
    println!(
        "Buffer currently holds: {:?}",
        str::from_utf8(&buffer[0..bytes_read]).unwrap()
    );
}
