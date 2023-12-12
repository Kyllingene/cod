# Cod

Cod is a lightweight, (almost) zero-dependency command-line drawing utility. It
works much like a basic C library, where you call functions to produce effects
on the screen, rather than use a struct or some fancy macro syntax. *However*,
it provides several conveniences above the ordinary C lib:
- Namespacing, i.e. `cod::style::bold()` and `cod::color::with()`
    - A prelude, imports all submodules
- Closure-based styling, i.e. `style::with::bold(|| {...})`
- Basic input gathering, i.e. `read::key()` or `read::line()`
    - Optional, behind feature `input`
    - Enables (and exposes) a dependency on
      [`console`](https://crates.io/crates/console)

There are some examples in the `examples` directory, but as cod aims to be as
simple to use as possible, they aren't prioritized. Moreover, everything in cod
is well-documented, so it's arguably easier to just look through the docs!

*Warning:* most cod functions don't flush stdout, so if you run into issues, try that!

## Bold and faint

You may notice that while there are separate functions for *enabling* bold and
faint, they both share `de::weight`. That's because, true to ANSI form,
terminals and VTE's can't agree on it, so we're stuck with only one way to
disable both. That goes for both `with::bold` and `with::faint` as well!

Additionally, on some terminals, bold and faint are mutually exclusive. On
some, like Alacritty, they can coexist only if the text is uncolored. In
general, be careful when using both at the same time. You can use the
`bold-faint` example to test your terminal.

