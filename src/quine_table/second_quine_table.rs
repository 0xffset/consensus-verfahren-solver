use std::fmt::Display;

use crate::Die;

use super::{
    second_quine_table_cell::SecondQuineTableCell, second_quine_table_entry::SecondQuineTableEntry,
};

const DIE_TITLE: &str = "Die";

pub struct SecondQuineTable<const N: usize> {
    entries: Vec<SecondQuineTableEntry<N>>,
    table: Vec<Vec<SecondQuineTableCell>>,
}

impl<const N: usize> SecondQuineTable<N> {
    /// Solves the quine table for its prime implicants
    pub fn solve(&mut self) {}

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
        let mut terms = Vec::new();
        for entry in &entries {
            for &term in &entry.covers {
                if !terms.contains(&term) {
                    terms.push(term);
                }
            }
        }
        terms.sort();

        let mut table = vec![vec![SecondQuineTableCell::default(); entries.len()]; terms.len()];

        for (i, entry) in entries.iter().enumerate() {
            for &term in &entry.covers {
                table[i][term].entry = true;
            }
        }

        SecondQuineTable { entries, table }
    }
}

impl<const N: usize> From<Vec<Die<N>>> for SecondQuineTable<N> {
    fn from(entries: Vec<Die<N>>) -> Self {
        SecondQuineTable::from(
            entries
                .iter()
                .map(|&die| SecondQuineTableEntry::from(die))
                .collect::<Vec<SecondQuineTableEntry<N>>>(),
        )
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
