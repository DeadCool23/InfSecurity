use rand::{rng, seq::SliceRandom};

pub struct Rotor<T> {
    position: usize,
    alphabet_len: usize,

    forward_alphabet: Vec<T>,
    backward_alphabet: Vec<T>,
}

impl<T: Clone + Ord> Rotor<T> {
    pub fn from_alphabet(alphabet: &[T]) -> Self {
        let mut rng = rng();

        let mut sorted_alphabet = alphabet.to_vec();
        let mut shuffeled_alphabet = alphabet.to_vec();

        sorted_alphabet.sort();
        shuffeled_alphabet.shuffle(&mut rng);

        Rotor {
            alphabet_len: alphabet.len(),
            forward_alphabet: sorted_alphabet,
            backward_alphabet: shuffeled_alphabet,
            position: 0,
        }
    }

    pub fn from_config(config: &[T]) -> Self {
        let mut sorted_alphabet = config.to_vec();
        sorted_alphabet.sort();

        Rotor {
            alphabet_len: config.len(),
            forward_alphabet: sorted_alphabet,
            backward_alphabet: config.to_vec(),
            position: 0,
        }
    }

    pub fn get_config(&self) -> Vec<T> {
        self.backward_alphabet.clone()
    }

    pub fn forward(&self, input: &T) -> Option<T> {
        let index = self.forward_alphabet.iter().position(|x| x == input);

        index.map(|i| self.backward_alphabet[(i + self.position) % self.alphabet_len].clone())
    }

    pub fn backward(&self, input: &T) -> Option<T> {
        let index = self.backward_alphabet.iter().position(|x| x == input);

        index.map(|i| {
            self.forward_alphabet[(i + self.alphabet_len - self.position) % self.alphabet_len]
                .clone()
        })
    }

    pub fn is_at_init_position(&self) -> bool {
        self.position == 0
    }

    pub fn rotate(&mut self) {
        self.position = (self.position + 1) % self.alphabet_len
    }

    pub fn reset(&mut self) {
        self.position = 0
    }
}
