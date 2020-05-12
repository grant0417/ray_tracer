# Ray Tracer

![Cornell Box](imgs/cornell.png)
*Rendered at 1280x1280 with 4096 samples/pixel in 01:19:34 on a Ryzen R7 2700x.*

## About

A basic ray tracer that is based on [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
but in Rust.

Some features include:
 * Parallel execution of ray tracing
 * .obj loading
 * BVH acceleration
 * Command line interface
 * Multiple image output formats

Here are some more images from the ray tracer:

  
![Utah Teapot and Stanford Dragon](imgs/objs.png)
  
![Lots of Balls](imgs/image.png)

## How to use

To run the program clone the repository then run `cargo run --release -- file.png` or use
`cargo run --release -- --help` to get more info on how to use the program.
