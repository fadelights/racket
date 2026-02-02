use hound::{WavReader, WavSpec, WavWriter};
use rand::Rng;
use clap::Parser;

struct PinkNoiseGenerator {
    b0: f32,
    b1: f32,
    b2: f32,
    b3: f32,
    b4: f32,
    b5: f32,
    b6: f32,
}

impl PinkNoiseGenerator {
    fn new() -> Self {
        Self {
            b0: 0.0,
            b1: 0.0,
            b2: 0.0,
            b3: 0.0,
            b4: 0.0,
            b5: 0.0,
            b6: 0.0,
        }
    }

    fn generate(&mut self, rng: &mut impl Rng) -> f32 {
        let white: f32 = rng.gen_range(-1.0..1.0);

        self.b0 = 0.99886 * self.b0 + white * 0.0555179;
        self.b1 = 0.99332 * self.b1 + white * 0.0750759;
        self.b2 = 0.96900 * self.b2 + white * 0.1538520;
        self.b3 = 0.86650 * self.b3 + white * 0.3104856;
        self.b4 = 0.55000 * self.b4 + white * 0.5329522;
        self.b5 = -0.7616 * self.b5 - white * 0.0168980;

        let pink =
            self.b0 + self.b1 + self.b2 + self.b3 + self.b4 + self.b5 + self.b6 + white + 0.5362;
        self.b6 = white + 0.115926;

        pink * 0.11
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: String,
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let in_path = &args.input;
    let out_path = &args.output;

    let (mut samples, spec) = read_wav(in_path)?;

    let mut rng = rand::thread_rng();
    let mut pnkg = PinkNoiseGenerator::new();

    white(&mut samples, 0.01, &mut rng);
    pink(&mut samples, 0.05, &mut pnkg, &mut rng);
    distort(&mut samples, 2.0);
    tremolo(&mut samples, spec, 6.0, 0.2);
    telephone(&mut samples, 0.9);
    ring_modulate(&mut samples, spec, 20.0);
    bitcrush(&mut samples, 8);

    write_wav(out_path, &samples, spec)?;

    Ok(())
}

fn read_wav(path: &str) -> Result<(Vec<i16>, WavSpec), Box<dyn std::error::Error>> {
    let mut reader = WavReader::open(path)?;
    let spec = reader.spec();
    let mut samples = Vec::new();

    for sample in reader.samples::<i16>() {
        samples.push(sample?);
    }

    Ok((samples, spec))
}

fn write_wav(path: &str, samples: &[i16], spec: WavSpec) -> Result<(), Box<dyn std::error::Error>> {
    let mut writer = WavWriter::create(path, spec)?;

    for &sample in samples {
        writer.write_sample(sample)?;
    }
    writer.finalize()?;

    Ok(())
}

fn normalize(x: f32) -> i16 {
    x.clamp(i16::MIN as f32, i16::MAX as f32) as i16
}

fn white(samples: &mut [i16], level: f32, rng: &mut impl Rng) {
    for s in samples.iter_mut() {
        let original = *s as f32;
        let noise = rng.gen_range(-i16::MAX..=i16::MAX) as f32;

        let mix = original + (noise * level);
        *s = normalize(mix);
    }
}

fn pink(samples: &mut [i16], level: f32, pnkg: &mut PinkNoiseGenerator, rng: &mut impl Rng) {
    for s in samples.iter_mut() {
        let original = *s as f32;
        let noise = pnkg.generate(rng);
        let noise_scaled = noise * i16::MAX as f32 * level;

        let mix = original + noise_scaled;
        *s = normalize(mix);
    }
}

fn distort(samples: &mut [i16], gain: f32) {
    for s in samples.iter_mut() {
        let original = *s as f32;
        let amplified = original * gain;

        let distorted = (amplified / i16::MAX as f32).tanh() * i16::MAX as f32;
        *s = normalize(distorted);
    }
}

fn tremolo(samples: &mut [i16], spec: WavSpec, rate: f32, depth: f32) {
    let mut phase = 0.0f32;
    let phase_increment = 2.0 * std::f32::consts::PI * rate / spec.sample_rate as f32;

    for s in samples.iter_mut() {
        let original = *s as f32;
        let lfo = (phase.sin() + 1.0) / 2.0; // 0.0 to 1.0
        let modulation = 1.0 - (depth * (1.0 - lfo));

        let tremoloed = original * modulation; // silly name for an affected sample :)
        *s = normalize(tremoloed);

        phase += phase_increment;
        if phase > 2.0 * std::f32::consts::PI {
            phase -= 2.0 * std::f32::consts::PI;
        }
    }
}

fn telephone(samples: &mut [i16], alpha: f32) {
    let mut prev_sample = 0.0f32;

    for s in samples.iter_mut() {
        let original = *s as f32;
        let high_passed = original - prev_sample * alpha;

        let telephoned = high_passed * 1.5;
        *s = normalize(telephoned);

        prev_sample = original;
    }
}

fn ring_modulate(samples: &mut [i16], spec: WavSpec, frequency: f32) {
    let mut phase = 0.0f32;
    let phase_increment = 2.0 * std::f32::consts::PI * frequency / spec.sample_rate as f32;

    for s in samples.iter_mut() {
        let original = *s as f32;

        let modulated = original * phase.sin();
        *s = normalize(modulated);

        phase += phase_increment;
        if phase > 2.0 * std::f32::consts::PI {
            phase -= 2.0 * std::f32::consts::PI;
        }
    }
}

fn bitcrush(samples: &mut [i16], bits: u32) {
    let shift = 16u32.saturating_sub(bits.min(16));
    let step = 1i32 << shift;

    for s in samples.iter_mut() {
        let original = *s as i32;

        let crushed = (original / step) * step;
        *s = crushed.clamp(i16::MIN as i32, i16::MAX as i32) as i16; // same as normalize, but for i32
    }
}
