use rand::Rng;

pub trait MemoryTrait {
    fn get(&self, position: u16) -> u8;

    fn set(&mut self, position: u16, value: u8) -> u8;

    fn _randomise(data: &mut Vec<u8>) {
        let mut rng = rand::rng();

        for byte in data.iter_mut() {
            *byte = rng.random();
        }
    }
}