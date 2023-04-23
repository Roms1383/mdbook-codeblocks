# process and open example
demo: (test 'true')

# process example, optionally opening browser
test OPEN='false':
  rm -rf example/book
  cargo build --release
  cargo install --path .
  cd example && RUST_LOG=info mdbook build{{ if OPEN == "true" { " --open" } else { "" } }} .
  cargo uninstall mdbook-codeblocks

# bundle binary (CI)
bundle-tar TAG TARGET:
  mv {{ join( "target", TARGET, "release", if TARGET == 'x86_64-pc-windows-gnu' { "mdbook-codeblocks.exe" } else { "mdbook-codeblocks" } ) }} {{ if TARGET == 'x86_64-pc-windows-gnu' { "mdbook-codeblocks.exe" } else { "mdbook-codeblocks" } }}
  tar -czvf mdbook-codeblocks-{{TAG}}-{{TARGET}}.tar.gz \
  {{ if TARGET == 'x86_64-pc-windows-gnu' { "mdbook-codeblocks.exe" } else { "mdbook-codeblocks" } }}

# bundle binary (CI)
bundle-7z TAG TARGET:
  7z a "mdbook-codeblocks-{{TAG}}-{{TARGET}}.zip" "{{ join( `pwd`, "target", TARGET, "release", "mdbook-codeblocks.exe" ) }}"

bundle-zip TAG TARGET:
  zip "mdbook-codeblocks-{{TAG}}-{{TARGET}}.zip" "{{ join( `pwd`, "target", TARGET, "release", "mdbook-codeblocks.exe" ) }}"
