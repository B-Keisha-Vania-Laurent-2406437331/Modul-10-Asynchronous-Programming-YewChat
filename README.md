# YewChat 💬

> Source code for [Let’s Build a Websocket Chat Project With Rust and Yew 0.19 🦀](https://fsjohnny.medium.com/lets-build-a-websockets-project-with-rust-and-yew-0-19-60720367399f)

## Install

1. Install the required toolchain dependencies:
   ```npm i```

2. Follow the YewChat post!

## Branches

This repository is divided to branches that correspond to the blog post sections:

* main - The starter code.
* routing - The code at the end of the Routing section.
* components-part1 - The code at the end of the Components-Phase 1 section.
* websockets - The code at the end of the Hello Websockets! section.
* components-part2 - The code at the end of the Components-Phase 2 section.
* websockets-part2 - The code at the end of the WebSockets-Phase 2 section.

## Experiment 3.1: Original Code

### Screenshot
![Experiment 3.1](asets/images/ori_code.png)

The application consists of two parts: a JavaScript WebSocket server and a Rust/Yew frontend client compiled to WebAssembly. When a user opens the app, they enter their username on the login page and click "Go Chatting!" to enter the chat room. Messages typed by any user are sent to the WebSocket server, which broadcasts them to all connected clients in real time. Each user is displayed in the Users panel on the left side with their avatar and name.
