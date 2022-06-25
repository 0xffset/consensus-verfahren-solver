use consensus_solver::{self, ConesnsusTable, ConsensusTableEntry, Die};

#[test]
pub fn unused_dont_care() {
    let mut dice = Vec::new();
    let mut dont_cares = Vec::new();

    dice.push(Die::new("01100"));
    dont_cares.push(Die::new("00000"));

    let mut table: ConesnsusTable<5> = ConesnsusTable::new(dice, dont_cares);
    table.solve();

    let compare: ConesnsusTable<5> = ConesnsusTable::from(vec![
        ConsensusTableEntry {
            num: Some(0),
            creators: Vec::new(),
            die: Die::new("00000"),
            covered: Some(0),
            dont_care: true,
        },
        ConsensusTableEntry {
            num: Some(1),
            creators: Vec::new(),
            die: Die::new("01100"),
            covered: None,
            dont_care: false,
        },
    ]);

    assert!(compare.to_string() == table.to_string());
}

#[test]
pub fn unused_merged_dont_care() {
    let mut dice = Vec::new();
    let mut dont_cares = Vec::new();

    dice.push(Die::new("01100"));
    dont_cares.push(Die::new("00000"));
    dont_cares.push(Die::new("10000"));

    let mut table: ConesnsusTable<5> = ConesnsusTable::new(dice, dont_cares);
    table.solve();

    let compare: ConesnsusTable<5> = ConesnsusTable::from(vec![
        ConsensusTableEntry {
            num: Some(0),
            creators: Vec::new(),
            die: Die::new("00000"),
            covered: Some(3),
            dont_care: true,
        },
        ConsensusTableEntry {
            num: Some(1),
            creators: Vec::new(),
            die: Die::new("10000"),
            covered: Some(3),
            dont_care: true,
        },
        ConsensusTableEntry {
            num: Some(2),
            creators: Vec::new(),
            die: Die::new("01100"),
            covered: None,
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(3),
            creators: vec![1, 0],
            die: Die::new("-0000"),
            covered: Some(3),
            dont_care: true,
        },
    ]);

    assert!(compare.to_string() == table.to_string());
}
