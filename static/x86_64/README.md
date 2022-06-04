<h1 align="center">「🧱」About TTWAF</h1>

<p align="center"><img src="assets/logo.png"></p>

<p align="center">TTWAF, or Test This WAF, is a Web Application Firewall (WAF) bypass testing tool. You can test a list of payloads like XSS, LFI, RCE, SQLI and find one that can bypass WAF. TTWAF is built in Rust, this is great for faster testing.</ṕ>

## Usage:

<p align="center"><img src="assets/demo.png"></p>

## Help:

<p align="center"><img src="assets/help.png"></p>

# How to Install:

## Requirements:

```
# First install rust
$ rustup install stable
$ rustup target add x86_64-unknown-linux-musl
$ sudo apt install libssl-dev
```

## Installation:

```
$ git clone https://github.com/MrEmpy/TTWAF
$ cd TTWAF
$ make
$ cp ttwaf /usr/bin
```

