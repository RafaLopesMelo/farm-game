use crate::world::coords::Coords;

use super::perlin::PerlinNoise;

pub struct FractalNoise {
    noise: PerlinNoise,
}

impl FractalNoise {
    pub fn new() -> Self {
        return Self {
            noise: PerlinNoise::new(),
        };
    }

    pub fn generate(&self, coords: Coords, octaves: u8, persistence: f32, lacunarity: f32) -> f32 {
        let mut noise = 0.0;
        let mut amplitude = 1.0;
        let mut frequency = 0.005;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            noise += self.noise.generate(coords, frequency) * amplitude;
            max_value += amplitude;

            amplitude *= persistence; // Decrease amplitude
            frequency *= lacunarity; // Increase frequency
        }

        return noise / max_value;
    }
}
