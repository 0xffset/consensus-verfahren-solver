use crate::{
    die::Die,
    table_entry::{print_consensus_table, ConsensusTableEntry},
};

/// Solves the given function in die form using the consensus method
pub fn solve_using_consensus_method<const N: usize>(dice: Vec<Die<N>>, dont_care: Vec<Die<N>>) {
    let mut count = 0;

    // convert all dont care dice to TabelEntries
    let mut entries: Vec<ConsensusTableEntry<N>> = dont_care
        .iter()
        .map(|&die| {
            let entry = ConsensusTableEntry::new(Some(count), die, true);
            count += 1;
            entry
        })
        .collect();

    // save the highest dont_care num
    let dont_care_bound = count;
    
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
            // if the element is covered by another element, skip it
            if entries[curr].covered.is_some() {
                continue;
            }

            // compare all previous elements to the current one
            for comp in 0..curr {
                // if the element is covered by another element, skip it
                if entries[comp].covered.is_some() {
                    continue;
                }

                if let Some(mut new_entry) =
                    ConsensusTableEntry::merge(&entries[curr], &entries[comp])
                {
                    // check if die is being covered
                    for entry in &entries {
                        if entry.covered.is_none() && entry.die.covers(new_entry.die) {
                            new_entry.covered = Some(entry.num.unwrap());
                            break;
                        }
                    }

                    // if the die is covered don't assign a number and continue
                    if new_entry.covered.is_some() {
                        continue;
                    }

                    // assign die number since its not covered
                    // this loop assumes that there is at least one die with a number
                    for entry in entries.iter().rev() {
                        if entry.num.is_some() {
                            new_entry.num = Some(entry.num.unwrap() + 1);
                            break;
                        }
                    }

                    // check if new entry covers the other ones
                    for entry in &mut entries {
                        if entry.covered.is_none() && new_entry.die.covers(entry.die) {
                            entry.covered = Some(new_entry.num.unwrap());
                        }
                    }

                    entries.push(new_entry);
                }
            }

            curr += 1;
        }

        for num in 0..dont_care_bound {
            let mut found = false;
            // check if entry is used
            for entry in &entries {
                if entry.creators.contains(&num) {
                    found = true;
                    break;
                }
            }

            // else mark it as covered by itself since its optional
            if !found {
                entries[num].covered = Some(num);
            }
        }

        print_consensus_table(&entries);
    }
}
