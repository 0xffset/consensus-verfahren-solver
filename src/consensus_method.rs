use crate::{
    die::Die,
    table_entry::{print_consensus_table, ConsensusTableEntry},
};

/// Solves the given function in die form using the consensus method
pub fn solve_using_consensus_method<const N: usize>(dice: Vec<Die<N>>, dont_care: Vec<Die<N>>) {
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

    if dice.is_empty() {
        print_consensus_table(&entries);
        return;
    }

    // convert all normal dice to TableEntries
    for entry in dice.iter().map(|&die| {
        let entry = ConsensusTableEntry::new(Some(count), die, false);
        count += 1;
        entry
    }) {
        entries.push(entry);
    }

    if entries.len() > 1 {
        let mut curr = 1;
        // start at the second element, and walk down the list
        while curr < entries.len() {
            // if the element is covered by another element (and not dont care by itself), skip it
            match entries[curr].covered {
                Some(val) => match entries[curr].num {
                    Some(num) if val != num => continue,
                    _ => {}
                },
                _ => {}
            }

            // compare all previous elements to the current one
            for comp in 0..curr {
                // if the element is covered by another element (and not dont care by itself), skip it
                match entries[comp].covered {
                    Some(val) => match entries[comp].num {
                        Some(num) if val != num => continue,
                        _ => {}
                    },
                    _ => {}
                }

                if let Some(mut new_entry) =
                    ConsensusTableEntry::merge(&entries[curr], &entries[comp])
                {
                    // check if die is being covered (and not dont care by itself)
                    for entry in &entries {
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
                    for entry in entries.iter().rev() {
                        if entry.num.is_some() {
                            new_entry.num = Some(entry.num.unwrap() + 1);
                            break;
                        }
                    }

                    // check if new entry covers the other ones (and not dont care by itself)
                    for entry in &mut entries {
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

                    entries.push(new_entry);
                }
            }

            curr += 1;
        }
    }
    
    print_consensus_table(&entries);
}
