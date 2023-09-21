# Building Lenker

You will need the rust compiler suite to build this project. The recommended tool is `rustup` to manage the compiler suite.

```sh
snap install --classic rustup
rustup toolchain install stable
```

You can then perform `cargo build` to produce a binary at `target/debug/lenker` to try it out.

You can install the release by running `./install.sh`, which installs `lenker` and `lenkerx` to `~/.local/bin`. You may need to add this location to your PATH.

