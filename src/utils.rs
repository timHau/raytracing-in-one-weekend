use rand::Rng;

pub(crate) fn random_float() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}
