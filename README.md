# Re🅱️is

This is a simple CLI Redis inspired project that supports the `GET`, `SET`, and `INCR` commands.

## Run it

1. Have rust installed: https://rustup.rs/
1. Run this:

```sh
❯ cargo run
   Compiling rebis v0.1.0 (/Users/user/rebis)
    Finished dev [unoptimized + debuginfo] target(s) in 1.52s
     Running `target/debug/rebis`
> set foo 1
OK
> set hello world
OK
> get hello
world
> incr foo
(integer) 2
```

## TODO

- [ ] Address various `todo!()` in the codebase
- [ ] Better module structure
- [ ] Break up each action into its own file
- [ ] Export a wasm lib so we can run this in the browser
