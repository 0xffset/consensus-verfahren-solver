use std::fmt::Display;

use crate::Die;

use super::ConsensusTableEntry;

const NUM_TITLE: &str = "Num.";
const CREATOR_TITLE: &str = "Created by";
const DIE_TITLE: &str = "Die";
const COVERED_TITLE: &str = "Covered by";

/// A consensus table used for simplifying the input function
pub struct ConesnsusTable<const N: usize> {
    entries: Vec<ConsensusTableEntry<N>>,
}

impl<const N: usize> ConesnsusTable<N> {
    pub fn new(dice: Vec<Die<N>>, dont_care: Vec<Die<N>>) -> Self {
        let mut count = 0;

        // convert all dont care dice to TabelEntries
        let mut entries: Vec<ConsensusTableEntry<N>> = Vec::new();

        if !dont_care.is_empty() {
            entries = dont_care
                .iter()
                .map(|&die| {
                    let mut entry = ConsensusTableEntry::new(Some(count), die, true);
                    // mark optional entries as covered by themselves
                    // helps with saving on looping through the entire vector
                    // to check for unused optional dont care dice
                    entry.covered = Some(count);
                    count += 1;
                    entry
                })
                .collect();
        }

        // convert all normal dice to TableEntries
        for entry in dice.iter().map(|&die| {
            let entry = ConsensusTableEntry::new(Some(count), die, false);
            count += 1;
            entry
        }) {
            entries.push(entry);
        }

        ConesnsusTable { entries }
    }

    pub fn solve(&mut self) {
        if self.entries.len() > 1 {
            let mut curr = 1;
            // start at the second element, and walk down the list
            while curr < self.entries.len() {
                // if the element is covered by another element (and not dont care by itself), skip it
                match self.entries[curr].covered {
                    Some(val) => match self.entries[curr].num {
                        Some(num) if val != num => continue,
                        _ => {}
                    },
                    _ => {}
                }

                // compare all previous elements to the current one
                for comp in 0..curr {
                    // if the element is covered by another element (and not dont care by itself), skip it
                    match self.entries[comp].covered {
                        Some(val) => match self.entries[comp].num {
                            Some(num) if val != num => continue,
                            _ => {}
                        },
                        _ => {}
                    }

                    if let Some(mut new_entry) =
                        ConsensusTableEntry::merge(&self.entries[curr], &self.entries[comp])
                    {
                        // check if die is being covered (and not dont care by itself)
                        for entry in &self.entries {
                            match entry.covered {
                                Some(val) => match entry.num {
                                    Some(num) if val == num && entry.die.covers(new_entry.die) => {
                                        new_entry.covered = Some(entry.num.unwrap());
                                        break;
                                    }
                                    _ => {}
                                },
                                _ if entry.die.covers(new_entry.die) => {
                                    new_entry.covered = Some(entry.num.unwrap());
                                    break;
                                }
                                _ => {}
                            }
                        }

                        // if the die is covered don't assign a number and continue
                        if new_entry.covered.is_some() {
                            continue;
                        }

                        // assign die number since its not covered
                        for entry in self.entries.iter().rev() {
                            if entry.num.is_some() {
                                new_entry.num = Some(entry.num.unwrap() + 1);
                                break;
                            }
                        }

                        // check if new entry covers the other ones (and not dont care by itself)
                        for entry in &mut self.entries {
                            match entry.covered {
                                Some(val) => match entry.num {
                                    Some(num) if val == num && new_entry.die.covers(entry.die) => {
                                        entry.covered = Some(new_entry.num.unwrap())
                                    }
                                    _ => {}
                                },
                                _ if new_entry.die.covers(entry.die) => {
                                    entry.covered = Some(new_entry.num.unwrap())
                                }
                                _ => {}
                            }
                        }

                        self.entries.push(new_entry);
                    }
                }

                curr += 1;
            }
        }
    }
}

impl<const N: usize> From<Vec<ConsensusTableEntry<N>>> for ConesnsusTable<N> {
    fn from(entries: Vec<ConsensusTableEntry<N>>) -> Self {
        ConesnsusTable { entries }
    }
}

impl<const N: usize> Display for ConesnsusTable<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut biggest_num = NUM_TITLE.len();
        let mut biggest_creators = CREATOR_TITLE.len();
        let mut biggest_covered = COVERED_TITLE.len();
        let mut die_len = DIE_TITLE.len();

        die_len = if !self.entries.is_empty() {
            self.entries[0].die.to_string().len()
        } else {
            // only print header since the table is empty
            writeln!(
                f,
                " {:biggest_num$} ┃ {:biggest_creators$} ┃ {:die_len$} ┃ {:biggest_covered$}",
                NUM_TITLE, CREATOR_TITLE, DIE_TITLE, COVERED_TITLE
            )?;
            writeln!(
                f,
                "{}╋{}╋{}╋{}",
                "━".repeat(biggest_num + 2),
                "━".repeat(biggest_creators + 2),
                "━".repeat(die_len + 2),
                "━".repeat(biggest_covered + 2)
            )?;
            return Ok(());
        };

        for entry in &self.entries {
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

        for entry in &self.entries {
            writeln!(
                f,
                "{}",
                entry.to_string(biggest_num, biggest_creators, die_len, biggest_covered)
            )?;
        }

        Ok(())
    }
}
