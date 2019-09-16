# Disclaimers
`GLOLA` is purely experimental and not likely to be adapted to your hardware or your software configuration, if you do not know what you're doing here : run away

# What is GLOLA
A safe and easy to use wrapper for huge led matrix backed with `OLA`.
## Use case
Imagine that you got very huge led matrix *ex: 400\*400* that is controlled using `DMX` packet, for using it as a screen with a reasonable refresh rate you must splite your led matrix into multiple `artnet` univers and be aware of the hardware limitations to avoid visual glitch. This library reduce the complexity by providing simple API and ensure the refresh rate is respected.

# Wrapper
`GLOLA` is made from native code and his API is quite simple so you can use it in most programming languages but some wrapper are already made for NodeJS, C/C++ and RUST

# Examples
## CLI
The `cli` example is a simple command line tool that use the library.
to work propely the cli must be configured using environment variable `CONFIG` that point to a json configuration file.
Here is an example config (you can get it from `cargo run --example cli -- default-cfg`)
### Generate config
```json
{
    "dmx_size": 400,
    "width": 40,
    "height": 40,
    "univer_height": 10,
    "channel_per_pixel": 10,
    "color_mode": "RGBA",
    "displacement": "Snake",
    "direction": "Horizontal",
    "orientation": "BottomRight"
}

```
## CLI
A simple CLI tool is provided with glola, its provide some feature like addressing debug or media transcoding.
```
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    dump    Inspect a configuration file and dump address map.
    gif     Send a GIF file frame by frame to the matrix.
    help    Prints this message or the help of the given subcommand(s)
```