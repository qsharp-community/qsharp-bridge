use qsharp_bridge::noise::{Noise, PauliNoiseDistribution};

#[test]
fn test_ideal_noise_to_distribution() {
    let noise = Noise::Ideal;
    let dist = noise.to_distribution().unwrap();
    assert_eq!(dist.x, 0.0);
    assert_eq!(dist.y, 0.0);
    assert_eq!(dist.z, 0.0);
}

#[test]
fn test_pauli_noise_to_distribution() {
    let pauli_dist = PauliNoiseDistribution::new(0.1, 0.2, 0.3).unwrap();
    let noise = Noise::Pauli {
        noise: pauli_dist.clone(),
    };
    let dist = noise.to_distribution().unwrap();
    assert_eq!(dist.x, 0.1);
    assert_eq!(dist.y, 0.2);
    assert_eq!(dist.z, 0.3);
}

#[test]
fn test_bit_flip_noise_to_distribution() {
    let noise = Noise::BitFlip { p: 0.1 };
    let dist = noise.to_distribution().unwrap();
    assert_eq!(dist.x, 0.1);
    assert_eq!(dist.y, 0.0);
    assert_eq!(dist.z, 0.0);
}

#[test]
fn test_phase_flip_noise_to_distribution() {
    let noise = Noise::PhaseFlip { p: 0.2 };
    let dist = noise.to_distribution().unwrap();
    assert_eq!(dist.x, 0.0);
    assert_eq!(dist.y, 0.0);
    assert_eq!(dist.z, 0.2);
}

#[test]
fn test_depolarizing_noise_to_distribution() {
    let noise = Noise::Depolarizing { p: 0.3 };
    let dist = noise.to_distribution().unwrap();
    assert!((dist.x - 0.1).abs() < 1e-9);
    assert!((dist.y - 0.1).abs() < 1e-9);
    assert!((dist.z - 0.1).abs() < 1e-9);
}

#[test]
fn test_pauli_distribution_new_valid() {
    let dist = PauliNoiseDistribution::new(0.1, 0.2, 0.3);
    assert!(dist.is_ok());
}

#[test]
fn test_pauli_distribution_new_invalid_negative() {
    let dist = PauliNoiseDistribution::new(-0.1, 0.2, 0.3);
    assert!(dist.is_err());
}

#[test]
fn test_pauli_distribution_new_invalid_sum() {
    let dist = PauliNoiseDistribution::new(0.5, 0.6, 0.1);
    assert!(dist.is_err());
}
