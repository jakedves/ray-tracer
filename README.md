# Ray Tracer in Rust

A ray tracer written in Rust, based off the C++ version found in [*Ray Tracing in One Weekend*](https://raytracing.github.io/books/RayTracingInOneWeekend.html).

I used the C++ version and tried to translate it to Rust to learn the language more, rather than just using the Rust version of the book.

Not a perfect implementation but a very fun project, and here is my final render:

![A beautiful image with lots of different and reflective spheres](https://github.com/jakedves/ray-tracer/blob/main/render.png)

## What I Learnt

- Borrowing, Ownership, Move (& Copy) semantics in Rust
- File writing in Rust
- Ray tracing techniques
- `Box<T>` in and dynamic dispatch in Rust
- Materials, fuzzy reflection, refraction
- Dielectrics
- Schlick approximation
- Defocus blur and thin-lens approximation
