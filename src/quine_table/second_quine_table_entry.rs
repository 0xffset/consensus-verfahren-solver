use crate::{Die, DieSide};

pub struct SecondQuineTableEntry<const N: usize> {
    pub die: Die<N>,
    pub covers: Vec<usize>,
}

impl<const N: usize> SecondQuineTableEntry<N> {
    /// Checks if the die covers a term
    pub fn covers(&self, term: usize) -> bool {
        self.covers.contains(&term)
    }
}

impl<const N: usize> From<Die<N>> for SecondQuineTableEntry<N> {
    fn from(die: Die<N>) -> Self {
        let mut dont_cares = Vec::new();
        // safe all dont care indices
        for (i, side) in die.content.iter().enumerate() {
            match side {
                DieSide::DontCare => dont_cares.push(i),
                _ => {}
            }
        }

        let mut covers = Vec::new();

        if !dont_cares.is_empty() {
            let initial_die = die.as_binary_string();

            // store the length of the longest number
            let pad = format!("{:b}", 1 << dont_cares.len()).len() + 1;

            // generate 2^n numbers where n is the amount of dont cares
            for i in 0..(1 << dont_cares.len()) {
                let mut temp = initial_die.clone();

                // convert number to binary and pad with leading 0
                // skip the 0b prefix
                // replace all dont cares with a digit of the number
                for (j, c) in format!("{i:#0pad$b}").chars().skip(2).enumerate() {
                    temp.remove(dont_cares[j]);
                    temp.insert(dont_cares[j], c);
                }

                // can be safely unwrapped since die are always in a valid state
                covers.push(usize::from_str_radix(temp.as_str(), 2).unwrap());
            }
        } else {
            // only covers one term

            // can be safely unwrapped since die are always in a valid state
            covers.push(usize::from_str_radix(die.as_binary_string().as_str(), 2).unwrap());
        }

        SecondQuineTableEntry { die, covers }
    }
}
