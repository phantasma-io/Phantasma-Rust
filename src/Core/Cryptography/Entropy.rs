use rand::Rng;

pub struct Entropy;

impl Entropy {
    pub fn get_random_bytes(target_length: usize) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let random_bytes: Vec<u8> = (0..target_length).map(|_| rng.gen()).collect();
        random_bytes
    }
}
