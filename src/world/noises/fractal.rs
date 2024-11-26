use crate::world::coords::Coords2D;

use super::{cache::NoiseCache, perlin::PerlinNoise};

pub struct FractalNoise {
    noise: PerlinNoise,
}

impl FractalNoise {
    pub fn new() -> Self {
        return Self {
            noise: PerlinNoise::new(),
        };
    }

    pub fn generate(
        &self,
        coords: Coords2D,
        desc: &FractalNoiseGenerationDescriptor,
        cache: Option<&NoiseCache>,
    ) -> f32 {
        let mut noise = 0.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;
        let mut tmp_frequency = desc.frequency;

        for _ in 0..desc.octaves {
            noise += self.noise.generate(coords, tmp_frequency) * amplitude;
            max_value += amplitude;

            amplitude *= desc.persistence; // Decrease amplitude
            tmp_frequency *= desc.lacunarity; // Increase frequency
        }

        return noise / max_value;
    }
}

pub struct FractalNoiseGenerationDescriptor {
    /// How many rounds of noise will be used to compose the final value
    pub octaves: u8,
    /// For which factor the amplitude will be multiplied in each octave
    pub persistence: f32,
    /// For which factor the frequency will be multiplied in each octave
    pub lacunarity: f32,
    /// Start frequency, used to calculate the first octave
    pub frequency: f32,
}
