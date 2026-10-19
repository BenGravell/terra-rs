Country flags from [Flagpedia / FlagCDN](https://flagpedia.net/download/api), based on Wikimedia Commons artwork.

`atlas.rs` contains 64 × 48 RGBA cells in `countries.txt` order.
Each country is labeled with its code; each line is a row of 64 `0xRRGGBBAA` pixels.
The Rust array is included at compile time and converted to RGBA bytes once at runtime.
Straight flags are downloaded from `https://flagcdn.com/w80/{code}.png`, then fitted into each cell with their original
proportions and transparent padding.
Bundled for offline rendering without an image decoder or emoji font.
