pub mod cfg;

mod reflector;
mod rotor;

use reflector::Reflector;
use rotor::Rotor;

pub struct Enigma<T> {
    commutator: Option<Reflector<T>>,
    reflector: Reflector<T>,
    rotors: Vec<Rotor<T>>,
}

impl<T: Clone + Eq + Ord> Enigma<T> {
    pub fn from_alphabet(
        alphabet: &[T],
        rotors_cnt: u8,
        with_commutator: bool,
    ) -> Result<Self, &str> {
        let commutator = if with_commutator {
            Some(Reflector::from_alphabet(alphabet))
        } else {
            None
        };

        let reflector = Reflector::from_alphabet(alphabet);
        let rotors = (0..rotors_cnt)
            .map(|_| Rotor::from_alphabet(alphabet))
            .collect();

        Ok(Enigma {
            commutator,
            reflector,
            rotors,
        })
    }

    pub fn from_config<'a>(
        commutator_config: Option<&'a [T]>,
        reflector_config: &'a [T],
        rotors_configs: &'a [Vec<T>],
    ) -> Result<Self, &'a str> {
        let commutator = if let Some(cfg) = commutator_config {
            Some(Reflector::from_config(cfg))
        } else {
            None
        };

        let reflector = Reflector::from_config(reflector_config);
        let rotors = (0..rotors_configs.len())
            .map(|i| {
                if i == 0 {
                    Rotor::from_config(&rotors_configs[i])
                } else if rotors_configs[i - 1].len() == rotors_configs[i].len() {
                    Rotor::from_config(&rotors_configs[i])
                } else {
                    panic!("Different sizes of rotor configs")
                }
            })
            .collect();

        Ok(Enigma {
            commutator,
            reflector,
            rotors,
        })
    }

    pub fn get_config(&self) -> (Option<Vec<T>>, Vec<T>, Vec<Vec<T>>) {
        (
            self.commutator.as_ref().map(|c| c.get_config()),
            self.reflector.get_config(),
            self.rotors.iter().map(|rotor| rotor.get_config()).collect(),
        )
    }

    fn encrypt_symbol(&mut self, symbol: &T) -> Result<T, &'static str> {
        let mut encrypt_symb = symbol.clone();

        if let Some(commutator) = &self.commutator {
            encrypt_symb = commutator
                .reflect(&encrypt_symb)
                .ok_or("Symbol not in alphabet")?;
        }

        for rotor in &self.rotors {
            encrypt_symb = rotor
                .forward(&encrypt_symb)
                .ok_or("Symbol not in alphabet")?;
        }

        encrypt_symb = self
            .reflector
            .reflect(&encrypt_symb)
            .ok_or("Symbol not in alphabet")?;

        for rotor in self.rotors.iter().rev() {
            encrypt_symb = rotor
                .backward(&encrypt_symb)
                .ok_or("Symbol not in alphabet")?;
        }

        if let Some(commutator) = &self.commutator {
            encrypt_symb = commutator
                .reflect(&encrypt_symb)
                .ok_or("Symbol not in alphabet")?;
        }

        self.rotate_rotors();

        Ok(encrypt_symb.clone())
    }

    pub fn encrypt(&mut self, buf: &[T]) -> Result<Vec<T>, (usize, &'static str)> {
        let mut ebuf = Vec::with_capacity(buf.len());

        for (i, symb) in buf.iter().enumerate() {
            ebuf.push(self.encrypt_symbol(symb).map_err(|err_str| (i, err_str))?);
        }

        Ok(ebuf)
    }

    pub fn decrypt(&mut self, buf: &[T]) -> Result<Vec<T>, (usize, &'static str)> {
        self.encrypt(buf)
    }

    fn rotate_rotors(&mut self) {
        for i in 0..self.rotors.len() {
            if i == 0 {
                self.rotors[i].rotate();
            } else if self.rotors[i - 1].is_at_init_position() {
                self.rotors[i].rotate();
            }
        }
    }

    pub fn reset(&mut self) {
        for rotor in &mut self.rotors {
            rotor.reset();
        }
    }
}
