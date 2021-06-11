# A layout generator for [river](https://github.com/ifreund/river)

kile is a layout client for river. It comes with a few layouts like deck, vertical and horizontal split you can [use](https://gitlab.com/snakedye/kile/-/blob/main/doc/kile.1.scd#L98) with [layout values](https://gitlab.com/snakedye/kile/-/blob/main/doc/kile.1.scd#L33) to create new unique dynamic layouts.

Layout examples are provided in the [contrib](https://gitlab.com/snakedye/kile/-/blob/main/doc/contrib) file.

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
