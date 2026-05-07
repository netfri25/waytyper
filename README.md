# waytyper
simple wayland client that types any UTF-8 it gets from stdin

### Requirements
 - [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
 - compositor with `input-method-unstable-v2` support (look [here](https://wayland.app/protocols/input-method-unstable-v2#compositor-support) if your compositor supports it)

### Installation
```shell
cargo install --git https://github.com/netfri25/waytyper
```
this should install the binary `waytyper` to your cargo bin path, which by default is `$HOME/.cargo/bin`
