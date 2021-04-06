# A tiling layout generator for [river](https://github.com/ifreund/river)

## How does it work?

Kile is entirely configured through river-options.

### Help
`kile --help`.

## Building

```shell
git clone https://gitlab.com/snakedye/kile.git
cd kile
cargo build --release
```

## Roadmap

- [x] Implement the dwindle layout
- [x] Per tag layout
- [ ] Basic window rules
- [ ] Draw titlebar on tabbed window
- [ ] Manual tiling
