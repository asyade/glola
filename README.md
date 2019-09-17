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
#### Available parametters

- color_mode: `rgb`, `rgba`
- orientation: `TopLeft`, `TopRight`, `BottomLeft`, `BottomRight`

## CLI
A simple CLI tool is provided with glola, its provide some feature like addressing debug or media transcoding.
```
Loads a GIF image and sends its frames from an infinite loop.

USAGE:
    gli gif [FLAGS] [OPTIONS] -c <config> -g <gif>

FLAGS:
        --help       
            Prints help information

    -h               
            Hexdump every outgoing packet on the standard output.

    -V, --version    
            Prints version information

    -w               
            Parse and display in a window the outgoing packet content.


OPTIONS:
    -c <config>            
            The matrix configuration is done using a json file, to check a configuration you can use the subcommand
            dump, example config for a 10x10 matrix that use DMX512 : `{"dmx_size": 512,"width": 30,"height":
            30,"univer_height": 10,"channel_per_pixel": 10,"color_mode": "RGBA","displacement": "Snake","direction":
            "Horizontal","orientation": "TopLeft"}`
    -g <gif>               
            GIF file to parse.

    -m <multiplier>        
            Pixel size multiplier display on the screen (use low value for large matrix and increase it for small one).

```

### Example

```
cargo run --release --example gli -- gif -c ./examples/config/40x40.json -g ./examples/imgs/40X40.gif -w -m 10
```