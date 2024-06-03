# Brainf*ck-rust

## About
Brainfuck-rust is a [Brainf*ck](https://en.wikipedia.org/wiki/Brainfuck) interpreter I wrote to learn Rust. For more samples, see [brain-lang's examples](https://github.com/brain-lang/brainfuck/tree/master/examples). They have some pretty cool programs on there!

## Usage
1. Follow the [build steps](https://github.com/megabyte6/brainfuck-rust?tab=readme-ov-file#build).
1. Run the executable through the command line using one of the following commands ():
    - On Linux/macOS:
        - `./target/release/bf run ./examples/mandelbrot.bf`
        - `./target/release/bf build -o ./mandelbrot ./examples/mandelbrot.bf`
        - `./target/release/bf help`
    - On Windows:
        - `.\target\release\bf.exe run .\examples\mandelbrot.bf`
        - `.\target\release\bf.exe build -o .\mandelbrot.exe .\examples\mandelbrot.bf`
        - `.\target\release\bf.exe help`

## Build
1. [Install](https://www.rust-lang.org/tools/install) rust.
1. Clone this repository.
    - If you have Git installed, run `git clone https://github.com/megabyte6/brainfuck-rust.git`
    - Otherwise:
        1. Click on the "Code" button near the top of the page.
        1. Click on "Download ZIP".
        1. Extract zip folder.
1. Navigate to the downloaded directory
1. Run `cargo build --release`
1. The compiled binary will be in `./target/release/`. It will be named `bf` or `bf.exe` depending on your operating system.

## License
[The MIT License](https://opensource.org/licenses/MIT)

## Thanks
- [Alexander Overvoorde](https://github.com/Overv) as [bf](https://github.com/Overv/bf) was my inspiration for this project.
