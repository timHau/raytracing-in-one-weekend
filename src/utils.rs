use rand::Rng;

pub(crate) fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub(crate) fn random_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}
