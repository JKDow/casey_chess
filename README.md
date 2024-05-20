# Casey Chess Engine 
## Description 
Casey is a chess engine written entirely in the Rust programming language. It is a personal project to explore the principles of chess programming and how Rust can be applied in this field. In the early versions, the code may not be optimal as research into chess programming techniques has been kept to a minimum to see what can be created independently. Future updates will incorporate optimizations based on further research.
## Features
- A board implemented with a 2D vector. 
- All logic for piece movement and game play. 
- FEN support. 
- Legal move generation.
- Engine plays random moves. 
- Limited functionality UCI implementation. 
## Installation 
Binaries can be found on the release page or the project can be built using the rust compiler. 

To build the project follow these steps.
```bash
git clone https://github.com/JKDow/casey_chess.git
cd casey chess
cargo build --release
```
The binary will then be found in the created target directory. 
## Usage
When running the binary it will work using UCI. To begin run `uci` in the console and use the `position` and `go` commands to interact with it. Or connect it to a chess GUI that supports UCI. 
## Future plans 
- Setup better CLI utility 
- Create testing suite for the engine 
- Implement basic evaluation function
- Create bench marking suite for the engine 
- Optimize engine functionality 
- Improve evaluation function 
