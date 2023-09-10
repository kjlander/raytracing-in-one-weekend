# Ray Tracing in One Weekend with Rust
An implementation of [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust. 

While done as an exercise to learn Rust and become more familiar with its  
features, this implementation does not attempt to completely Rust-ify the  
code presented in the book, as this is one of the first projects I've done  
with Rust. It also does not take advantage of external libraries, such as  
[_Glam_](https://crates.io/crates/glam) for a Vec3 implementation, because  
I wanted to implement these features myself as a learning exercise.

The code follows the C++ from the book pretty closely, and after reviewing  
some other Rust implementations, I can see a lot of places for improvement,  
such as making more use of Options rather than following the mutable argument  
pattern seen often in the source material, and making more use of iterators.  
Overall, I think this was a good learning exercise.