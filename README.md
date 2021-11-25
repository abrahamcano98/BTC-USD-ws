# BTC-USD-ws
Web socket client developed in rust to get BTC-USD real time data from Binance.

# Installing
### Via cargo
```
cargo install --git https://github.com/abrahamcano98/BTC-USD-ws
```
### Via git
```
git clone https://github.com/abrahamcano98/BTC-USD-ws
```


# Building
## Prerequisites
The requirements are explained at "install.txt".

Then, execute (cargo only):
```
$ cargo build
```
# Basic usage
To connect via websocket for 10 seconds and store the data in a file ("output/BTC-USD.txt") run (in the parent directory):
### Via Cargo:
```
$ cargo run --mode=cache
```
### Via executable client
```
$ /.simple --mode=cache
```
To show the collected data, run:

### Via Cargo:
```
$ cargo run read
```
### Via executable client
```
$ /.simple --mode=read
```
# Visualizations
When you want to generate a plot of both raw trade streams and aggregate, in src/main.rs line 16, set the third argument to be true:
```rust
core::write_data("BTC-USD.txt",data,true);
```
The plots are located in the directory "output".



