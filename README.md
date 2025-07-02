# Rustyblocks

## What is Rustyblocks?

Rustblocks is basically the try to reimplement dwmblocks with Rust.

## But why?

Because I wanted to do something with Rust and and what is better to take a simple already existing
project and reimplement it.

I also keep the KISS principle in mind so that everything should be clear and simple.

## How to run it?

Install the whole rust toolchain in your distro (you should know how to do that).
After that just clone the respository and run

```
cargo build
```

Put the binary into ~/.local/bin and add ~/.local/bin to your PATH variable of your favourite shell (the same
here you should know how to do that otherwise go to Youtube and watch some of this excellent Linux introduction
videos)

## Can I contribute?

Yes off course ... just open a pull request but at the end this program should stay simple.

## Why did you use XCB instead of XLib?

Because I'm inexperienced and XCB seem to me the more modern way to talk to the X server.

## Why did you write it for X and not for Wayland?

Because I use X because it's well tested and I want my graphics system to just work.
