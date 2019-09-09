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
  "ordering": "NextCloumnFromBottom", // Can be NextCloumnFromBottom or NextCloumnFromTop see glola::Matrix
  "dmx_size": 512,
  "cloumn": 10,
  "row": 10
}

```
### Show address map
Once your environment is set you can run the following command to verify your led addressing
```bash
cargo run --example cli -- dump-addr
```
### Decode GIF
```bash
cargo run --example cli -- gif-loop [GIF FILE PATH]
```