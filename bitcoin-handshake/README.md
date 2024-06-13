# Bitcoin Handshake in Rust

This is an example project for my blog post [Bitcoin Handshake Guide for Rust](https://medium.com/@jannden/0b9aab2d869b).

## How to use it

1. Use [rustup](https://rustup.rs/) to install the latest stable version of Rust.
2. Clone the repository.
3. Open the directory of this project with `cd bitcoin-handshake`
4. Start it up with `cargo run`.

Make sure you are connecting to a Bitcoin node that is accepting incoming connections. You can find a list of nodes [here](https://bitnodes.io/nodes/). Update the constant `BITCOIN_NODE` in `src/main.rs` with the IP address of the node you want to connect to.

## Example output

```shell
Connected to Bitcoin node at 66.35.84.30:8333
- Protocol Version: 70016
- Services: 3081
- Timestamp: 1718279410
- Receiver Address Services: 0
- Receiver Address IP: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 255, 62, 176, 70, 117]
- Receiver Address Port: 63751
- Local Address Services: 3081
- Local Address IP: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
- Local Address Port: 0
- Nonce: 15839917738839508669
- User Agent: "/Satoshi:27.0.0/"
- Start Height: 847755
- Relay: 1
Received 'version' message
Sent 'verack' message
Received 'wtxidrelay' message: []
Received 'sendaddrv2' message: []
Received 'verack' message
Handshake complete.
Sent 'getblocks' message
Sent 'ping' message
```

## TODO
Although the handshake is complete, nodes don't reply to the `getblocks` and `ping` messages. Needs further investigation.