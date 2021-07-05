# Rspace
rspace is a mini space shooter written in Rust

It uses the following programming patterns
- Game states
- Entity Component System
- Events
- Resource caching

Still, it's missing a scene graph or quadtree to optimize collision detection.

# Installing dependencies
sudo apt install libsdl2-dev libsdl2-mixer-dev libsdl2-image-dev libsdl2-ttf-dev libsdl2-gfx-dev

# Getting the source code
git clone https://github.com/Scorbutics/rspace.git

# Running the game
cd rspace/
cargo run rspace