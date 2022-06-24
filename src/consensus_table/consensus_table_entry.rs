use crate::Die;

/// Represents an entry in the table used in the consensus method
pub struct ConsensusTableEntry<const N: usize> {
    pub num: Option<usize>,
    pub creators: Vec<usize>,
    pub die: Die<N>,
    pub covered: Option<usize>,
    pub dont_care: bool,
}

impl<const N: usize> ConsensusTableEntry<N> {
    pub fn new(num: Option<usize>, die: Die<N>, dont_care: bool) -> Self {
        ConsensusTableEntry {
            num,
            creators: Vec::new(),
            die,
            covered: None,
            dont_care,
        }
    }

    /// Attempts to merge two table entries according to the die's merge implementation. <br>
    /// If successful it returns a new table entry with the creators set, else none
    pub fn merge(a: &ConsensusTableEntry<N>, b: &ConsensusTableEntry<N>) -> Option<Self> {
        if let Some(merged_die) = a.die.merge(b.die) {
            return Some(ConsensusTableEntry {
                num: None,
                creators: vec![a.num.unwrap(), b.num.unwrap()],
                die: merged_die,
                covered: None,
                dont_care: false,
            });
        }

        None
    }
}

impl<const N: usize> ConsensusTableEntry<N> {
    /// Converts the entry to string form in the form of ` <num> | <creators> | <die> | <covered> `
    pub fn to_string(
        &self,
        pad_num: usize,
        pad_creators: usize,
        pad_die: usize,
        pad_covered: usize,
    ) -> String {
        let covered = match self.covered {
            Some(val) => match self.num {
                Some(num) if num == val => "X".to_string(),
                Some(_) => format!("⊆ {val}"),
                None => format!("⊆ {val}"),
            },
            None => "".to_string(),
        };

        let num = match self.num {
            Some(val) => val.to_string(),
            None => "".to_string(),
        };

        format!(
            " {:pad_num$} ┃ {:pad_creators$} ┃ {:pad_die$} ┃ {:pad_covered$} ",
            num,
            self.creators
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", "),
            self.die.to_string(),
            covered,
        )
    }
}
