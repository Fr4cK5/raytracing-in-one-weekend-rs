# Ray Tracing in One Weekend

This is an implementation of the book [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html). (Book v4.0.1; Oct 23rd, 2024)

For an overview of all other available resources, check out their [io-page](https://raytracing.github.io/).

# Usage

```bash
git clone https://github.com/Fr4cK5/raytracing-in-one-weekend-rs ./rtiow
cd ./rtiow
cargo run -r
```

After running it, you'll find a file `test-img.pmm` in the project's root directory.
To look at it either use [GIMP](https://www.gimp.org/) or some online PPM-Viewer tool.

# An Example
This was rendered with following quality settings.
- 2560x1440 Resolution
- 1000 Samples/px
- 50 Bounces

![final-render](https://github.com/Fr4cK5/raytracing-in-one-weekend-rs/blob/master/final-render.jpeg?raw=true)
