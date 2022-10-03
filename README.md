# Alternative svgbob preprocessor for mdbook

[![Crates.io](https://img.shields.io/crates/v/mdbook-svgbob2)](https://crates.io/crates/mdbook-svgbob2)
[![GitHub last commit](https://img.shields.io/github/last-commit/matthiasbeyer/mdbook-svgbob2)](https://github.com/matthiasbeyer/mdbook-svgbob2)
[![License](https://img.shields.io/github/license/matthiasbeyer/mdbook-svgbob2)](https://github.com/matthiasbeyer/mdbook-svgbob2/blob/master/LICENSE)

A preprocessor for [mdbook](https://github.com/rust-lang/mdBook) to convert
`svgbob` codeblocks into nice SVG images using
[svgbob](https://github.com/ivanceras/svgbob).

This crate serves as an alternative to
[mdbook-svgbob](https://github.com/fzzr-/mdbook-svgbob) which was a big
inspiration while writing the code for this crate.

The reason why I created this was that the original uses outdated dependencies
and seems to break tables some reason, and it requires the source to have Unix
line endings as it breaks with Windows line endings. I also wanted to learn how
to write mdbook preprocessors and had a usecase for an updated svgbob
integration.

## Installation

Install with cargo:

```sh
cargo install mdbook-svgbob2
```

Or to install from the git mirror:

```sh
cargo install --git https://github.com/matthiasbeyer/mdbook-svgbob2.git
```

## Usage

Add this to your cargo.toml:
```toml
[preprocessor.svgbob2]
```

Optionally add any arguments for the
[svgbob settings](https://docs.rs/svgbob/0.6.2/svgbob/buffer/fragment/struct.Settings.html):

```toml
[preprocessor.svgbob2]
font_size = "14"
font_family = "monospace"
fill_color = "black"
background = "transparent"
stroke_color = "var(--fg)" # variable from the mdbook css files
stroke_width = "2.0"
scale = "8.0"
enhance_circuitries = "true"
include_backdrop = "true"
include_styles = "true"
include_defs = "true"
merge_line_with_shapes = "false"

# this is a non-svgbob custom setting
font_color = "var(--fg)"
```

The above are the default settings. Most of them are the svgbob defaults, with
the exception of `background` and `stroke_color` which have been changed to
better fit mdbook.

svgbob currently does not support changing the font color (although there are
[feature requests](https://github.com/ivanceras/svgbob/issues/78) for this to be
added), and because a pure black font color doesn't work well with different
mdbook themes, the ability to change the font color was added.

Simply include `svgbob` codeblocks in some chapter:

````md
```svgbob
       .---.
      /-o-/--
   .-/ / /->
  ( *  \/
   '-.  \
      \ /
       '
```
````

and it should display as a nicely rendered svg when rendered with mdbook.

