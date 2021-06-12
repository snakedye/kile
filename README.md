# A layout generator for [river](https://github.com/ifreund/river)

kile is a layout client for river. 

Through a lisp like syntax, users can define new dynamic layouts from existing ones. 

Layout examples are provided in the [contrib](https://gitlab.com/snakedye/kile/-/blob/main/contrib/layout) file.
The documentation is in the man page. 

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
