use rand::{rng, seq::SliceRandom};

#[allow(dead_code)]
pub struct StdReflector<T> {
    alphabet: Vec<T>,
}

#[allow(dead_code)]
impl<T: Clone + Eq> StdReflector<T> {
    pub fn from_alphabet(alphabet: &[T]) -> Result<Self, &str> {
        if alphabet.len() % 2 != 0 {
            return Err("Error: Can't be odd alphabet");
        }
        let mut rng = rng();
        let mut cipher = alphabet.to_vec();
        cipher.shuffle(&mut rng);

        Ok(StdReflector { alphabet: cipher })
    }

    pub fn from_config(config: &[T]) -> Result<Self, &str> {
        if config.len() % 2 != 0 {
            return Err("Error: Can't be odd alphabet");
        }

        Ok(StdReflector {
            alphabet: config.to_vec(),
        })
    }

    pub fn get_config(&self) -> Vec<T> {
        self.alphabet.clone()
    }

    pub fn reflect(&self, input: &T) -> Option<T> {
        let index = self.alphabet.iter().position(|x| x == input);

        index.map(|i| self.alphabet[if i % 2 == 0 { i + 1 } else { i - 1 }].clone())
    }
}

pub struct Reflector<T> {
    alphabet: Vec<T>,
}

impl<T: Clone + Eq> Reflector<T> {
    pub fn from_alphabet(alphabet: &[T]) -> Self {
        let mut rng = rng();
        let mut cipher = alphabet.to_vec();
        cipher.shuffle(&mut rng);

        Reflector { alphabet: cipher }
    }

    pub fn from_config(config: &[T]) -> Self {
        Reflector {
            alphabet: config.to_vec(),
        }
    }

    pub fn get_config(&self) -> Vec<T> {
        self.alphabet.clone()
    }

    pub fn reflect(&self, input: &T) -> Option<T> {
        let index = self.alphabet.iter().position(|x| x == input);

        index.map(|i|
            if i == self.alphabet.len() - 1 && self.alphabet.len() % 2 != 0 {
                self.alphabet[i].clone()
            } else {
                self.alphabet[if i % 2 == 0 { i + 1 } else { i - 1 }].clone()
            }
        )
    }
}
