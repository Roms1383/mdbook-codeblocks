# mdbook-codeblocks

![build](https://github.com/Roms1383/mdbook-codeblocks/actions/workflows/quality.yml/badge.svg) [![crates.io](https://img.shields.io/crates/v/mdbook-codeblocks.svg)](https://crates.io/crates/mdbook-codeblocks)

A preprocessor for [mdbook](https://rust-lang.github.io/mdBook/) which prepend a vignette above code blocks.

## usage

Customization is at its early stage, and let you define language's `icon`, `label`, `color` and `link`.

```toml
[book]
authors = ["Roms1383"]
title = "Code blocks preprocessor example"

[output.html]

[preprocessor.codeblocks]
renderers = ["html"]
cpp = { color = "#FFFF99", link = "https://wiki.redmodding.org/red4ext" }
redscript = { color = "tomato", link = "https://wiki.redmodding.org/redscript" }
rust = { color = "#ac4313", link = "https://github.com/jac3km4/red4ext-rs" }
lua = { link = "https://wiki.redmodding.org/cyber-engine-tweaks" }
swift = { label = "Swift code snippet", color = "skyblue" }
```

Here's what it will produce:

![example](example.png)

You can actually find it in `example` folder.

⚠️ also, do not forget to embed [FontAwesome css](https://cdnjs.com/libraries/font-awesome), see `example/theme/head.hbs`.

## roadmap

This tool is very rudimentary, but contributions are welcomed!

Especially for the HTML/CSS :)

Likewise it only supports a small subset of 5 languages so far, please add more!

## attribution

This tool uses icons from [FontAwesome](https://fontawesome.com/) by default.
