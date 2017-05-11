I wanted to try out gfx-rs and also realized I'd never written a Mandelbrot set renderer in glsl so I did both and this is that.

Controls are arrow keys to move and pg up/down to zoom in/out. We don't get infinite zoom due to the finite precision of floats. I might try adding support for doubles but it seems like gfx doesn't support double-precision uniforms maybe. Actually, only the gfx::Global type, which are uniform variables, don't support doubles. The ConstantBuffer struct provides doubles for uniform buffers/blocks it appears.

Run the program by installing Rust and Cargo, and running `cargo run` or `cargo run --release` in this directory.

![screenshot of mandlebrot fractal rendered by program](https://raw.githubusercontent.com/boustrophedon/mandelbrot-gfx-rs/master/screenshot.png)
![screenshot of zoomed in mandlebrot fractal rendered by program](https://raw.githubusercontent.com/boustrophedon/mandelbrot-gfx-rs/master/screenshot_zoom.png)
