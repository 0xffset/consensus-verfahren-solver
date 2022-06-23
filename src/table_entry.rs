use crate::die::Die;

const NUM_TITLE: &str = "Num.";
const CREATOR_TITLE: &str = "Created by";
const DIE_TITLE: &str = "Die";
const COVERED_TITLE: &str = "Covered by";

/// Represents an entry in the table used in the consensus method
pub struct ConsensusTableEntry<const N: usize> {
    pub num: Option<usize>,
    pub creators: Vec<usize>,
    pub die: Die<N>,
    pub covered: Option<usize>,
}

impl<const N: usize> ConsensusTableEntry<N> {
    pub fn new(num: Option<usize>, die: Die<N>) -> Self {
        ConsensusTableEntry {
            num,
            creators: Vec::new(),
            die,
            covered: None,
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
            });
        }

        None
    }

    /// Converts the entry to string form in the form of ` <num> | <creators> | <die> | <covered> `
    pub fn to_string(
        &self,
        pad_num: usize,
        pad_creators: usize,
        pad_die: usize,
        pad_covered: usize,
    ) -> String {
        let covered = match self.covered {
            Some(val) => format!("⊆ {val}"),
            None => "".to_string(),
        };

        let num = match self.num {
            Some(val) => val.to_string(),
            None => "".to_string(),
        };

        format!(
            " {:pad_num$} | {:pad_creators$} | {:pad_die$} | {:pad_covered$} ",
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

/// Prints the entire table
pub fn print_consensus_table_entries<const N: usize>(entries: &Vec<ConsensusTableEntry<N>>) {
    let mut biggest_num = 0;
    let mut biggest_creators = 0;
    let mut biggest_covered = 0;
    let mut die_len = entries[0].die.to_string().len();
    for entry in entries {
        match entry.num {
            Some(val) if val > biggest_num => biggest_num = val,
            Some(_) => {}
            None => {}
        }

        if !entry.creators.is_empty() {
            let len = entry
                .creators
                .iter()
                .map(ToString::to_string)
                .collect::<String>()
                .len();
            if len > biggest_creators {
                biggest_creators = len
            }
        }

        match entry.covered {
            Some(val) if val > biggest_covered => biggest_covered = val,
            Some(_) => {}
            None => {}
        }
    }

    if biggest_num < NUM_TITLE.len() {
        biggest_num = NUM_TITLE.len();
    }
    if biggest_covered < COVERED_TITLE.len() {
        biggest_covered = COVERED_TITLE.len();
    }
    if biggest_creators < CREATOR_TITLE.len() {
        biggest_creators = CREATOR_TITLE.len();
    }
    if die_len < DIE_TITLE.len() {
        die_len = DIE_TITLE.len();
    }

    println!(
        " {:biggest_num$} | {:biggest_creators$} | {:die_len$} | {:biggest_covered$}",
        NUM_TITLE, CREATOR_TITLE, DIE_TITLE, COVERED_TITLE
    );
    for entry in entries {
        println!(
            "{}",
            entry.to_string(biggest_num, biggest_creators, die_len, biggest_covered)
        );
    }
}
