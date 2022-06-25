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

    /// This function checks if an entry in the table covers the passed entry and, if it does, sets the covered attribute accordingly
    fn table_covers_die(&self, subject: &mut ConsensusTableEntry<N>) {
        for entry in &self.entries {
            match entry.covered {
                Some(val) => match entry.num {
                    Some(num) if val == num && entry.die.covers(subject.die) => {
                        subject.covered = Some(entry.num.unwrap());
                        break;
                    }
                    _ => {}
                },
                _ if entry.die.covers(subject.die) => {
                    subject.covered = Some(entry.num.unwrap());
                    break;
                }
                _ => {}
            }
        }
    }

    /// Adds an entry to the table, marking all entries the new entry covers as covered by that
    fn add_entry_to_table(&mut self, subject: ConsensusTableEntry<N>) {
        // check if new entry covers the other ones (and not dont care by itself)
        for entry in &mut self.entries {
            match entry.covered {
                Some(val) => match entry.num {
                    Some(num) if val == num && subject.die.covers(entry.die) => {
                        entry.covered = Some(subject.num.unwrap())
                    }
                    _ => {}
                },
                _ if subject.die.covers(entry.die) => entry.covered = Some(subject.num.unwrap()),
                _ => {}
            }
        }

        self.entries.push(subject);
    }

    /// Checks if the element at index i is covered (ignoring dont cares that cover themselves)
    fn is_covered(&self, i: usize) -> bool {
        match self.entries[i].covered {
            Some(val) => match self.entries[i].num {
                Some(num) if val != num => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn solve(&mut self) {
        if self.entries.len() > 1 {
            let mut curr = 1;
            // start at the second element, and walk down the list
            while curr < self.entries.len() {
                if self.is_covered(curr) {
                    continue;
                }

                // compare all previous elements to the current one
                for comp in 0..curr {
                    if self.is_covered(comp) {
                        continue;
                    }

                    if let Some(mut new_entry) =
                        ConsensusTableEntry::merge(&self.entries[curr], &self.entries[comp])
                    {
                        self.table_covers_die(&mut new_entry);

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

                        // mark the die as dont care if it was made by two dont cares
                        if self.entries[curr].dont_care && self.entries[comp].dont_care {
                            new_entry.dont_care = true;
                            new_entry.covered = Some(new_entry.num.unwrap());
                        }

                        self.add_entry_to_table(new_entry);
                    }
                }

                curr += 1;
            }
        }
    }

    fn entry_to_string(
        &self,
        entry: &ConsensusTableEntry<N>,
        pad_num: usize,
        pad_creators: usize,
        pad_die: usize,
        pad_covered: usize,
    ) -> String {
        let covered = match entry.covered {
            Some(val) => match entry.num {
                Some(num) if num == val => "X".to_string(),
                Some(_) => format!("⊆ {val}"),
                None => format!("⊆ {val}"),
            },
            None => "".to_string(),
        };

        let num = match entry.num {
            Some(val) => val.to_string(),
            None => "".to_string(),
        };

        format!(
            " {:pad_num$} ┃ {:pad_creators$} ┃ {:pad_die$} ┃ {:pad_covered$} ",
            num,
            entry.creators.map_or(String::new(), |creators| creators
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ")),
            entry.die.to_string(),
            covered,
        )
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

            if entry.creators.is_some() {
                let len = entry
                    .creators
                    .map_or(String::new(), |creators| {
                        creators
                            .iter()
                            .map(ToString::to_string)
                            .collect::<Vec<String>>()
                            .join(", ")
                    })
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

        for entry in &self.entries {
            writeln!(
                f,
                "{}",
                self.entry_to_string(
                    entry,
                    biggest_num,
                    biggest_creators,
                    die_len,
                    biggest_covered
                )
            )?;
        }

        Ok(())
    }
}
