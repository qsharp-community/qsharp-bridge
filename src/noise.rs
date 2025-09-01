use crate::sim::QsError;

#[derive(Debug, Clone)]
pub enum Noise {
    Ideal,
    Pauli {
        noise: PauliNoiseDistribution,
    },
    BitFlip {
        p: f64,
    },
    PhaseFlip {
        p: f64,
    },
    Depolarizing {
        p: f64,
    },
}

impl Noise {
    pub fn to_distribution(&self) -> Result<PauliNoiseDistribution, QsError> {
        match self {
            Noise::Ideal => PauliNoiseDistribution::new(0.0, 0.0, 0.0),
            Noise::Pauli { noise } => Ok(noise.clone()),
            Noise::BitFlip { p } => PauliNoiseDistribution::new(*p, 0.0, 0.0),
            Noise::PhaseFlip { p } => PauliNoiseDistribution::new(0.0, 0.0, *p),
            Noise::Depolarizing { p } => PauliNoiseDistribution::new(*p / 3.0, *p / 3.0, *p / 3.0),
        }
    }
}

impl Default for Noise {
    fn default() -> Self {
        Noise::Ideal
    }
}

#[derive(Debug, Clone)]
pub struct PauliNoiseDistribution {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PauliNoiseDistribution {
    pub fn new(x: f64, y: f64, z: f64) -> Result<Self, QsError> {
        if x < 0.0 || y < 0.0 || z < 0.0 || x + y + z > 1.0 {
            return Err(QsError::ErrorMessage {
                error_text:
                    "Invalid Pauli distribution: values must be non-negative and sum to <= 1.0"
                        .to_string(),
            });
        }
        Ok(Self { x, y, z })
    }
}
