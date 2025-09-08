use super::ConfigSerializer;
use std::{
    fs::File,
    io::{Error, ErrorKind, Read, Write},
};

const BYTE_CNT: usize = 256;

pub struct BinConfigSerializer;

impl ConfigSerializer<u8> for BinConfigSerializer {
    fn save_configs(
        file: &mut File,
        commutator_config: Option<&[u8]>,
        reflector_config: &[u8],
        rotors_configs: &[Vec<u8>],
    ) -> Result<(), Error> {
        for rotor_config in rotors_configs {
            if rotor_config.len() != BYTE_CNT {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Rotor config length mismatch",
                ));
            }
        }
    
        match commutator_config {
            Some(cfg) => {
                file.write_all(&[1u8])?;
                file.write_all(cfg)?;
            }
            None => {
                file.write_all(&[0u8])?;
            }
        }
    
        file.write_all(reflector_config)?;
    
        let rotors_cnt = rotors_configs.len();
        if rotors_cnt > 255 {
            return Err(Error::new(ErrorKind::InvalidInput, "Too many rotors"));
        }
        file.write_all(&[rotors_cnt as u8])?;
    
        for rotor_config in rotors_configs {
            file.write_all(rotor_config)?;
        }
    
        Ok(())
    }
    
    fn get_configs(file: &mut File) -> Result<(Option<Vec<u8>>, Vec<u8>, Vec<Vec<u8>>), Error> {
        let mut num_buf = [0; 1];
        file.read_exact(&mut num_buf)?;
        let is_with_commutator = num_buf[0];
    
        let mut reflector_config = vec![0; BYTE_CNT as usize];
        let commutator_config = if is_with_commutator != 0 {
            file.read_exact(&mut reflector_config)?;
            Some(reflector_config.clone())
        } else {
            None
        };
    
        file.read_exact(&mut reflector_config)?;
    
        file.read_exact(&mut num_buf)?;
        let rotors_cnt = num_buf[0];
    
        let mut rotors_configs = Vec::with_capacity(rotors_cnt as usize);
        for _ in 0..rotors_cnt {
            let mut irotor_config = vec![0; BYTE_CNT as usize];
            file.read_exact(&mut irotor_config)?;
            rotors_configs.push(irotor_config);
        }
    
        Ok((commutator_config, reflector_config, rotors_configs))
    }
}
