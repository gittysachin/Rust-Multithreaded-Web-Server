use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::time::Duration;
use hello::ThreadPool;

fn main(){
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        // Code structure if we could spawn the thread for each request
        // We can create our own ThreadPool instead 
        
        // A thread pool is a group of spawned threads that are waiting and ready to handle a task. When the program receives a new task, it assigns one of the threads in the pool to the task, and that thread will process the task. The remaining threads in the pool are available to handle any other tasks that come in while the first thread is processing.
        // When the first thread is done processing its task, it’s returned to the pool of idle threads, ready to handle a new task. A thread pool allows you to process connections concurrently, increasing the throughput of your server.
        // Rather than spawning unlimited threads, we’ll have a fixed number of threads waiting in the pool. As requests come in, they’ll be sent to the pool for processing. The pool will maintain a queue of incoming requests.

        // thread::spawn(|| {
        pool.execute(|| {
            handle_connection(_stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    // If the server receives a request that takes a long time to process, subsequent requests will have to wait until the long request is finished, even if the new requests can be processed quickly.

    // Open two browser windows: one for http://127.0.0.1:7878/ and the other for http://127.0.0.1:7878/sleep. If you enter the / URI a few times, as before, you’ll see it respond quickly. But if you enter /sleep and then load /, you’ll see that / waits until sleep has slept for its full 5 seconds before loading.

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 404 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}