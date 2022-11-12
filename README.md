Rust playground for wgpu, pixels, embedded-graphics running in the browser!
===============================================================================

### Building for web
Install the `wasm-pack` utility for your system. On Arch Linux, I used `sudo pacman -S wasm-pack` to get the tools. Installing via `cargo install wasm-pack` caused issues for me due to permissions.

Compile for web with `wasm-pack build --target web`.

Run by starting an http server, such as `python -m http.server`, and then open that up in your browser. Navigate to the `app/` folder to load the web app!

### Building for x86_64
Compile and run for x86_64 with `cargo run`

TODO:
* Update versions of `pixels` and `wgpu`, fix error that is referenced (here)[https://github.com/gfx-rs/wgpu/pull/2954].
* Get UI logic out of the platform-dependent attribute code.
   * Show the same thing on both platforms
