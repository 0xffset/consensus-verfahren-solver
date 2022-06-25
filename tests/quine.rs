use consensus_solver::{SecondQuineTable, SecondQuineTableEntry, Die};

#[test]
fn testt() {
    let table: SecondQuineTable<5> = SecondQuineTable::from(vec![
        SecondQuineTableEntry::from(Die::new("10111")),
        SecondQuineTableEntry::from(Die::new("0--11")),
        SecondQuineTableEntry::from(Die::new("00001")),
        SecondQuineTableEntry::from(Die::new("0-111")),
        
    ]);

    println!("{table}");
}