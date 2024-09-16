# Assault-Cube-minimap
A minimap to show player positions in Assault Cube 1.2.0.2

## Preview
![](./preview.gif)

## Getting started

### Clone the repo

```
https://github.com/Mikko-P/Assault-Cube-minimap.git
```

```
cd Assault-Cube-minimap
```

### Setup Node

```
cd src/backend
npm i net ws
node server.js
```

### Compile rust
#### Since Assault Cube is a x86 application we need to compile our binary to x86 as well

```
cd src/rust
rustup target add i686-pc-windows-msvc
cargo build --target i686-pc-windows-msvc
```

### Start the frontend in a live server
### Start assault cube 1.2.0.2
### Run the rust binary

## Q&A

### Q: [Why do you send data from Rust -> Node -> Frontend? Wouldn't it be faster to skip Node?]

**A:** [Yes, but you'd need to use a websocket crate, I use the Node server as a bridge to Websocket for frontend since I wanted to try it out]

---
