use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::net::TcpListener;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

type SharedState = Arc<Mutex<HashMap<String, (String, Option<SystemTime>)>>>;

fn main() -> std::io::Result<()> {
    let filename = "db.txt";
    let initial_state = load_from_file(filename).unwrap_or_else(|_| HashMap::new());
    let shared_state: SharedState = Arc::new(Mutex::new(initial_state));

    let shared_state_clone = Arc::clone(&shared_state);
    thread::spawn(move || loop {
        thread::sleep(Duration::from_secs(60)); // Save every 60 seconds
        let db = shared_state_clone.lock().unwrap();
        save_to_file(&db, "db.txt").expect("Failed to save database");
    });

    let listener = TcpListener::bind("127.0.0.1:6379")?;
    println!("Listening on port 6379");

    for stream in listener.incoming() {
        println!("New connection!");
        let stream = stream?;

        // Clone the Arc to increase the reference count
        let shared_state = Arc::clone(&shared_state);

        // Spawn a new thread for each connection
        thread::spawn(move || {
            handle_client(stream, shared_state).unwrap_or_else(|error| eprintln!("{:?}", error));
        });
    }
    Ok(())
}

fn handle_client(
    mut stream: std::net::TcpStream,
    shared_state: SharedState,
) -> std::io::Result<()> {
    loop {
        let mut buffer = [0; 1024];
        let bytes_read = stream.read(&mut buffer)?;
        if bytes_read == 0 {
            let db = shared_state.lock().unwrap();
            save_to_file(&db, "db.txt").expect("Failed to save database");
            break;
        }

        let command = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("Command received: {}", command);

        // Cut the command into parts and match them
        let parts: Vec<&str> = command.trim().split_whitespace().collect();
        match parts.as_slice() {
            ["PING"] => {
                stream.write_all(b"PONG\n")?;
            }
            ["ECHO", msg] => {
                let response = format!("{}\n", msg);
                stream.write_all(response.as_bytes())?;
            }
            ["SET", key, value] => {
                let mut db = shared_state.lock().unwrap();
                db.insert(key.to_string(), (value.to_string(), None)); // No expiry
                stream.write_all(b"OK\n")?;
            }
            ["SET", key, value, "EX", expiry] => {
                let expiry_duration = expiry.parse::<u64>().ok().map(|e| Duration::new(e, 0));
                if let Some(duration) = expiry_duration {
                    let mut db = shared_state.lock().unwrap();
                    let expiry_time = SystemTime::now() + duration;
                    db.insert(key.to_string(), (value.to_string(), Some(expiry_time)));
                    stream.write_all(b"OK\n")?;
                } else {
                    stream.write_all(b"Invalid expiry\n")?;
                }
            }
            ["GET", key] => {
                let mut db = shared_state.lock().unwrap();
                match db.get(*key) {
                    Some((_, Some(expiry))) if SystemTime::now() > *expiry => {
                        db.remove(*key);
                        stream.write_all(b"NOT FOUND\n")?;
                    }
                    Some((value, _)) => {
                        let response = format!("{}\n", value);
                        stream.write_all(response.as_bytes())?;
                    }
                    None => stream.write_all(b"NOT FOUND\n")?,
                }
            }
            _ => {
                let error_message = "Unknown command\n";
                stream.write_all(error_message.as_bytes())?;
            }
        }
    }
    Ok(())
}

fn save_to_file(
    db: &HashMap<String, (String, Option<SystemTime>)>,
    filename: &str,
) -> io::Result<()> {
    println!("Saving to file");
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    for (key, (value, expiry)) in db {
        if let Some(expiry) = expiry {
            let expiry_timestamp = expiry.duration_since(UNIX_EPOCH).unwrap().as_secs();
            writeln!(writer, "{} {} EX {}", key, value, expiry_timestamp)?;
        } else {
            writeln!(writer, "{} {}", key, value)?;
        }
    }
    Ok(())
}

fn load_from_file(filename: &str) -> io::Result<HashMap<String, (String, Option<SystemTime>)>> {
    let path = Path::new(filename);
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut db = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts.as_slice() {
            [key, value] => {
                db.insert(key.to_string(), (value.to_string(), None));
            }
            [key, value, "EX", expiry] => {
                let expiry = expiry
                    .parse::<u64>()
                    .ok()
                    .map(|e| UNIX_EPOCH + Duration::new(e, 0));
                db.insert(key.to_string(), (value.to_string(), expiry));
            }
            _ => {} // Ignore malformed lines for simplicity
        }
    }
    Ok(db)
}
