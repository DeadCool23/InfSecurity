pub mod bin;

pub use bin::BinConfigSerializer;
use std::{fs::File, io::Error};

pub trait ConfigSerializer<T> {
    fn save_configs(
        file: &mut File,
        commutator_config: Option<&[T]>,
        reflector_config: &[T],
        rotors_configs: &[Vec<T>],
    ) -> Result<(), Error>;

    fn get_configs(file: &mut File) -> Result<(Option<Vec<T>>, Vec<T>, Vec<Vec<T>>), Error>;
}