# Racket

Command-line audio processor written in Rust to apply various noise effects to WAV audio files.

## Features

Racket applies a chain of audio effects to transform your audio:

- **White Noise** - Adds subtle background noise
- **Pink Noise** - Adds warm, filtered noise using a pink noise generator
- **Distortion** - Applies harsh distortion with gain control
- **Tremolo** - Creates amplitude modulation for a wobbling effect
- **Telephone Effect** - High-pass filter to simulate old telephone quality
- **Ring Modulation** - Adds metallic, inharmonic overtones
- **Bitcrushing** - Reduces bit depth for a robot voice effect

## Setup

### Build from Source
```bash
# Clone or navigate to the repository
cd racket

# Build a development version (faster compile times but less optimized)
cargo build

# Build the release version
cargo build --release

# The binary will be available at ./target/debug/racket or ./target/release/racket
```

## Optional Dependencies
- To handle `mp3` files, you need to have `ffmpeg` installed on your system.
- To process files in parallel, you need to have `GNU parallel` installed.

## Usage

```bash
racket <input.wav> <output.wav>
```

But lets say your data is in `mp3` format. And you have a LOT of `mp3` files
to process. You can use the utility script `racket.sh` to convert them to `wav`,
process them, and convert them back to `mp3` again.

```bash
./racket.sh <path/to/mp3/dir>
```

The provided shell script will:
1. Receive as input a path to a directory containing `mp3` files.
2. Recursively find all `mp3` files in that directory.
3. Convert each `mp3` file to `wav` using `ffmpeg`.
4. Process each `wav` file with `racket` to apply the effects.
5. Convert the processed `wav` files back to `mp3` using `ffmpeg`.
6. Save the processed `mp3` files near the original files with a `_processed` suffix.

### Examples
```bash
./target/release/racket data/input/original.wav data/output/noisy.wav
```

```bash
./racket.sh data/input/mp3_files
```

### Help
```bash
racket --help
```

## Effect Parameters

The current effect chain uses the following parameters:

- White noise: 1% mix
- Pink noise: 5% mix
- Distortion: 2.0x gain
- Tremolo: 6 Hz rate, 20% depth
- Telephone: 0.9 high-pass filter coefficient
- Ring modulation: 20 Hz frequency
- Bitcrush: 8-bit depth

*Note: Parameters are currently hardcoded but can be modified in [main.rs](src/main.rs).*

## Technical Details

- **Input/Output**: WAV files only
- **Sample Format**: 16-bit PCM
- **Processing**: All effects are applied sequentially in-memory
- **Pink Noise**: Uses Paul Kellet's refined pink noise algorithm

## Dependencies

- [hound](https://crates.io/crates/hound) - WAV file I/O
- [rand](https://crates.io/crates/rand) - Random number generation
- [clap](https://crates.io/crates/clap) - Command-line argument parsing

## License

See [here](LICENSE) for license details.

## Enhancements

Some ideas for enhancements:

- [ ] Use a randomizer to randomly apply effects and parameters
- [ ] Make effect parameters configurable via CLI flags
- [ ] Add ability to enable/disable individual effects
- [ ] Support additional audio formats (especially MP3)
- [ ] Parallelize processing of multiple files using Rust itself instead of relying on GNU parallel

## References

- Paul Kellet's Pink Noise Algorithm: http://www.firstpr.com.au/dsp/pink-noise/
- Sample code for generating White, Pink, and Brown noises:
    https://github.com/joaocarvalhoopen/Audio_noise_WAV_generator_in_Rust
- Julius O. Smith III's Digital Audio Signal Processing:
    https://ccrma.stanford.edu/~jos/pasp/
- Distortion by soft clipping using `tanh`: http://gdsp.hf.ntnu.no/lessons/3/17/
- https://en.wikipedia.org/wiki/Tremolo
- https://en.wikipedia.org/wiki/High-pass_filter
- https://en.wikipedia.org/wiki/Ring_modulation
- https://en.wikipedia.org/wiki/Bitcrusher
- https://www.merriam-webster.com/dictionary/racket
