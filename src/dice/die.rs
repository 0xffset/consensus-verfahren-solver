use std::fmt::Display;

use super::DieSide;

/// Represents a boolean algebraic die
#[derive(Clone, Copy)]
pub struct Die<const N: usize> {
    pub content: [DieSide; N],
}

impl<const N: usize> Die<N> {
    pub fn new<S: AsRef<str>>(value: S) -> Self {
        if value.as_ref().len() != N {
            panic!("Value must be of length {}", N);
        }

        let mut content = [DieSide::DontCare; N];
        for (i, c) in value.as_ref().chars().enumerate() {
            match c {
                '0' => content[i] = DieSide::Zero,
                '1' => content[i] = DieSide::One,
                '-' => content[i] = DieSide::DontCare,
                _ => panic!("Invalid value, must only be 0s, 1s and -"),
            }
        }

        Die { content }
    }

    /// Tries to merge two dice. <br>
    /// Two dice are mergable when they differentiate in exactly 1 DieSide, ignoring DontCares. <br>
    /// If the dice are mergable the merged die will be returned, else none <br>
    ///
    /// Example: <br>
    /// (1 - 1 0) <br>
    /// (0 0 1 -) <br>
    /// (- 0 1 0) <=
    pub fn merge(&self, other: Die<N>) -> Option<Die<N>> {
        // check how many different die sides the dice have and ignore DontCares
        let mut diffs = Vec::new();
        for i in 0..N {
            if self.content[i] != DieSide::DontCare
                && other.content[i] != DieSide::DontCare
                && self.content[i] != other.content[i]
            {
                diffs.push(i);
            }
        }

        // if there is not exactly 1 difference, dice can't be merged
        if diffs.len() != 1 {
            return None;
        }

        let mut die = Die {
            content: [DieSide::Zero; N],
        };
        for i in 0..N {
            // if one die has a DontCare, copy the other die's value
            if self.content[i] == DieSide::DontCare {
                die.content[i] = other.content[i];
            } else if other.content[i] == DieSide::DontCare {
                die.content[i] = self.content[i];
            } else {
                // else just use either value, since they must be equal besides one difference
                die.content[i] = self.content[i];
            }
        }

        // Set the one difference to DontCare
        die.content[diffs[0]] = DieSide::DontCare;
        Some(die)
    }

    /// Checks if this die covers the passed die. <br>
    /// A die covers another die when all DieSides are the same, or DontCare
    ///
    /// Example: <br>
    /// `A = (- 0 1)` covers `B = (1 0 1)`
    pub fn covers(&self, other: Die<N>) -> bool {
        for i in 0..N {
            // if the current is not a DontCare and doesn't equal the others value, this die doesn't cover the other one
            if self.content[i] != DieSide::DontCare && self.content[i] != other.content[i] {
                return false;
            }
        }

        // all values are either the same, or this die covers the other dice values with DontCares
        true
    }

    pub fn as_binary_string(&self) -> String {
        self.content.iter().map(ToString::to_string).collect()
    }
}

impl<const N: usize> Display for Die<N> {
    /// Returns the die as string in the `(x1 x2 ... xN)` notation
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({})",
            self.content
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
