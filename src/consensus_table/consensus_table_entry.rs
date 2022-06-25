use crate::Die;

/// Represents an entry in the table used in the consensus method
pub struct ConsensusTableEntry<const N: usize> {
    pub num: Option<usize>,
    pub creators: Option<[usize; 2]>,
    pub die: Die<N>,
    pub covered: Option<usize>,
    pub dont_care: bool,
}

impl<const N: usize> ConsensusTableEntry<N> {
    pub fn new(num: Option<usize>, die: Die<N>, dont_care: bool) -> Self {
        ConsensusTableEntry {
            num,
            creators: None,
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
                creators: Some([a.num.unwrap(), b.num.unwrap()]),
                die: merged_die,
                covered: None,
                dont_care: false,
            });
        }

        None
    }
}
