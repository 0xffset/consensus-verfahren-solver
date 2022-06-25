use std::fmt::Display;

use super::second_quine_table_entry::SecondQuineTableEntry;

const DIE_TITLE: &str = "Die";

pub struct SecondQuineTable<const N: usize> {
    entries: Vec<SecondQuineTableEntry<N>>,
}

impl<const N: usize> SecondQuineTable<N> {
    fn entry_to_string(
        &self,
        entry: &SecondQuineTableEntry<N>,
        pad: usize,
        terms: &Vec<usize>,
    ) -> String {
        let mut ret = format!(" {} ", entry.die);

        for &term in terms {
            if entry.covers(term) {
                ret.push_str(format!("┃ {:pad$} ", 'X').as_str());
            } else {
                ret.push_str(format!("┃ {:pad$} ", ' ').as_str())
            }
        }

        ret
    }
}

impl<const N: usize> From<Vec<SecondQuineTableEntry<N>>> for SecondQuineTable<N> {
    fn from(entries: Vec<SecondQuineTableEntry<N>>) -> Self {
        SecondQuineTable { entries }
    }
}

impl<const N: usize> Display for SecondQuineTable<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut terms = Vec::new();

        let mut die_pad = DIE_TITLE.len();

        if self.entries.is_empty() {
            // only print header since the table is empty
            writeln!(f, " {:die_pad$} ┃ ", DIE_TITLE)?;
            writeln!(f, "{}╋━", "━".repeat(die_pad + 2))?;
            return Ok(());
        }

        let die_print_len = self.entries.first().unwrap().die.to_string().len();
        if die_pad < die_print_len {
            die_pad = die_print_len;
        }

        for entry in &self.entries {
            for &term in &entry.covers {
                if !terms.contains(&term) {
                    terms.push(term);
                }
            }
        }

        terms.sort();

        let pad = format!("{}", terms.last().unwrap()).len();

        write!(f, " {:die_pad$} ", DIE_TITLE)?;
        for term in &terms {
            write!(f, "┃ {:pad$} ", term)?;
        }
        writeln!(f)?;

        write!(f, "{}", "━".repeat(die_pad + 2))?;
        for _ in &terms {
            write!(f, "╋{}", "━".repeat(pad + 2))?;
        }
        writeln!(f)?;

        for entry in &self.entries {
            writeln!(f, "{}", self.entry_to_string(&entry, pad, &terms))?;
        }

        Ok(())
    }
}
