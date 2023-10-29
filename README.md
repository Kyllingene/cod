# cod - Command-line drawer

Cod is a simple, light, functional library for simple terminal I/O. It uses plain ANSI escape sequences to "draw" basic shapes and text to stdout; however, if the `input` feature is enabled, it uses the [`console`](https://crates.io/crates/console) crate to provide basic input gathering.

It's designed to be easy-to-use, as well as extremely light. It provides very basic utilities for moving the cursor, clearing the screen or a section thereof, displaying textual "sprites", and changing the foreground/background colors.

## Features
 - Rectangles, triangles, and lines
 - 8-bit and 24-bit color support
 - Text
 - Blitting `String`s, `Vec<String>`s, and `Vec<Vec<char>>`s
 - Basic feature-gated input gathering (`getch` and `getl` lookalikes)

