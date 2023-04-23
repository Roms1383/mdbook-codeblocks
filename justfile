# process and open example
demo: (test 'true')

# process example, optionally opening browser
test OPEN='false':
  rm -rf example/book
  cargo build --release
  cargo install --path .
  cd example && RUST_LOG=info mdbook build{{ if OPEN == "true" { " --open" } else { "" } }} .
  cargo uninstall mdbook-codeblocks