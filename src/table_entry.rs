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
                Some(num) => format!("⊆ {val}"),
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

/// Prints the entire table
pub fn print_consensus_table<const N: usize>(entries: &Vec<ConsensusTableEntry<N>>) {
    let mut biggest_num = NUM_TITLE.len();
    let mut biggest_creators = CREATOR_TITLE.len();
    let mut biggest_covered = COVERED_TITLE.len();
    let mut die_len = entries[0].die.to_string().len();
    if die_len < DIE_TITLE.len() {
        die_len = DIE_TITLE.len();
    }

    for entry in entries {
        match entry.num {
            Some(val) => {
                let num_len = val.to_string().len();
                if num_len > biggest_num {
                    biggest_num = num_len;
                }
            }
            _ => {}
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

        // only look at not optional dice, since optional dice will be marked with 'X'
        match entry.covered {
            Some(val) => match entry.num {
                Some(num) if num != val => {
                    let num_len = val.to_string().len();
                    if num_len > biggest_covered {
                        biggest_covered = num_len;
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    println!(
        " {:biggest_num$} ┃ {:biggest_creators$} ┃ {:die_len$} ┃ {:biggest_covered$}",
        NUM_TITLE, CREATOR_TITLE, DIE_TITLE, COVERED_TITLE
    );
    println!(
        "{}╋{}╋{}╋{}",
        "━".repeat(biggest_num + 2),
        "━".repeat(biggest_creators + 2),
        "━".repeat(die_len + 2),
        "━".repeat(biggest_covered + 2)
    );

    for entry in entries {
        println!(
            "{}",
            entry.to_string(biggest_num, biggest_creators, die_len, biggest_covered)
        );
    }
}
