# A layout generator for [river](https://github.com/ifreund/river)

kile is a layout client river. It comes with a few layouts like tabbed, vertical and horizontal split and dwindle. You can [combine](https://gitlab.com/snakedye/kile/-/blob/main/doc/kile.1.scd#L98) those layouts with [layout values](https://gitlab.com/snakedye/kile/-/blob/main/doc/kile.1.scd#L33) to create new unique layouts.

### Dependencies
- rust
- scdoc (optional)

## Building

[![Packaging status](https://repology.org/badge/vertical-allrepos/kile-wl.svg)](https://repology.org/project/kile-wl/versions)
```shell
git clone https://gitlab.com/snakedye/kile.git
cd kile
cargo build --release
```
