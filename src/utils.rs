use crate::vec::FloatType;

pub fn random_float() -> FloatType {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    rng.gen()
}

pub fn random_float_range(min: FloatType, max: FloatType) -> FloatType {
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    rng.gen_range(min..max)
}
