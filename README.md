# K's Rust Terminal Raycaster Thingy

Wanted to learn about raycasters, so I cobbled together a bunch of techniques I found from a few sources into a really simple example of raycasting. 


## Requirements

- **Rust:** It's in the name
- **Python:** For generating maps, you'll need Python.

## Getting Started

### 1. Clone the Repository

```bash
git clone https://github.com/khodges42/rustcaster.git
cd rustcaster
```

### 2. Install Dependencies

Make sure you have the required Rust and Python dependencies:

```bash
cargo build
```

### 3. Generate a Random Map (Optional)

To generate a random map file, use the Python script `gen_map.py` located in the `src` directory:

```bash
python src/gen_map.py
```

This will generate a file named `mapx.txt` by default, where x is the highest number of mapx in your directory. I didn't think that through, lol.


### 4. Run the Raycaster

To run the raycaster with the default map (`map.txt`):

```bash
cargo run
```

To specify a different map file:

```bash
cargo run -- path/to/your_map.txt
```

### 5. Controls

- **WASD:** Move, Look
- **Q:** Quit the application

## Code Overview

- **`src/main.rs`**: The main Rust source file containing the raycasting logic, rendering function, and input handling.
- **`src/gen_map.py`**: A Python script to generate random map files for testing.

## License

This project is licensed under the GNU General Public License v3.0 (GPL-3.0). See the [LICENSE](LICENSE) file for details. Shouts to stallman

## Acknowledgements

- I used a bunch of stuff from [this](https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/) tutorial
- Shouts to ChatGPT for giving me the math parts and the readme and the boilerplate code
- The terminal rendering and input handling are facilitated by the `crossterm` crate.

## Stuff I learned

- The level of simplicity here of calculating distance to the wall based on the Y in the render loop and then determining floor/ceiling is really clever
- (I say that because I didn't come up with it)
- Had I approached this without guides/chatgpt/etc, I would have probaby way overengineered this, it seems like this code is way too simple to work
