Country flags from [Flagpedia / FlagCDN](https://flagpedia.net/download/api), based on Wikimedia Commons artwork.

`atlas.rs` contains 32 × 24 RGBA cells in `countries.txt` order.
Each country is labeled with its code; each line is a row of 32 `0xRRGGBBAA` pixels.
The Rust array is included at compile time and converted to RGBA bytes once at runtime.
Flags keep their original proportions inside each cell.
Bundled for offline rendering without an image decoder or emoji font.
