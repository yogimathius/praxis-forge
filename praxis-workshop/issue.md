title: Investigate and improve developer experience and compile time for Rust/WASM frontend

body:

# Developer Experience Improvements for Rust/WASM Frontend

## Current Issues

- **Extremely large build artifacts**: Clean build is removing 3.2GiB of data
- **Slow compilation times**: Dependency upgrades and rebuilds take significantly longer than JS frameworks
- **Development iteration speed**: The compile-check-debug cycle is much slower than with JS frameworks

## Areas to Investigate

### Bundle Size Reduction

- [ ] Analyze what's contributing to the large build size
- [ ] Explore options for tree-shaking and dead code elimination
- [ ] Investigate if we can use smaller dependencies or feature flags to reduce included code

### Compile Time Optimization

- [ ] Look into using `cargo check` more strategically
- [ ] Explore incremental compilation options
- [ ] Consider splitting the codebase into smaller crates
- [ ] Investigate build caching strategies

### Developer Workflow Improvements

- [ ] Set up a development environment that minimizes full rebuilds
- [ ] Explore hot-reloading options for Leptos
- [ ] Document best practices for faster development cycles

## Resources to Explore

- [Rust WASM Working Group](https://github.com/rustwasm/team)
- [Leptos Performance Documentation](https://leptos.dev/docs)
- [wasm-bindgen optimization guide](https://rustwasm.github.io/wasm-bindgen/reference/optimize-size.html)
- [cargo-leptos](https://github.com/leptos-rs/cargo-leptos) for build optimization

## Goals

- Reduce build artifact size by at least 50%
- Decrease average compile time by 30%
- Establish a development workflow that allows for faster iteration

This is a learning opportunity to better understand Rust compilation, WASM optimization, and modern frontend development practices.
