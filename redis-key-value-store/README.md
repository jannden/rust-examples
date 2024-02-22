# Redis-like Key-Value Store in Rust

This is an example project for my blog post on [how to write a Redis-like key-value store](https://medium.com/@jannden/b48a94650c83).

## How to use it

1. Use [rustup](https://rustup.rs/) to install the latest stable version of Rust.
2. Clone the repository.
3. Open the directory of this project with `cd redis-key-value-store`
4. Start it up with `cargo run`.
5. Open another terminal and connect as per the instructions below.

### How to connect on Mac/Linux
Simply run `nc localhost 6379` in another terminal to connect to the server.

### How to connect on Windows
If using Powershell on Windows, it's a bit more complicated to test our program, but script such as this should do the trick:

```powershell
# Open a TCP connection to localhost on port 6379
$tcpClient = New-Object System.Net.Sockets.TcpClient("localhost", 6379)
$stream = $tcpClient.GetStream()
$writer = New-Object System.IO.StreamWriter($stream)
$reader = New-Object System.IO.StreamReader($stream)

# Send a command - adjust as needed
$writer.WriteLine("PING")
$writer.Flush()

# Read the response
$response = $reader.ReadLine()
echo "Response: $response"

# Cleanup
$writer.Close()
$reader.Close()
$tcpClient.Close()
```