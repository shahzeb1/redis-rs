# Redis

This is a simple CLI Redis inspired project that supports the `GET`, `SET`, and `INCR` commands.

## Run it

1. Have rust installed (if you don't, visit [rustup.rs](https://rustup.rs/))
1. Run the build via `cargo run`:

```sh
❯ cargo run
   Compiling redis v0.1.0 (/Users/user/redis)
    Finished dev [unoptimized + debuginfo] target(s) in 1.52s
     Running `target/debug/redis`
> set foo 1
OK
> set hello world
OK
> get hello
world
> incr foo
(integer) 2
```

Here's a primer on [Redis cheatsheet](https://developer.redis.com/howtos/quick-start/cheat-sheet/).

## Build the WASM artifacts:

Make sure you have [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) installed. Good [getting started guide](https://rustwasm.github.io/wasm-bindgen/introduction.html).

```sh
❯ wasm-pack --version
wasm-pack 0.12.1

❯ wasm-pack build
```

## TODO

- [x] Address various `todo!()` in the codebase
- [x] Better module structure, main file is too big
  - [x] Break up each action into its own file (module)
- [x] Export a wasm lib so we can run this in the browser
- [ ] Add new Redis actions. Feel free to contribute!
