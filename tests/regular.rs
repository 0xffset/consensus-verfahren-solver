use consensus_solver::{self, ConesnsusTable, ConsensusTableEntry, Die};

#[test]
pub fn test1() {
    let mut dice = Vec::new();
    let mut dont_cares = Vec::new();

    dice.push(Die::new("01100"));
    dice.push(Die::new("01101"));
    dice.push(Die::new("01110"));
    dice.push(Die::new("01111"));
    dice.push(Die::new("11101"));
    dice.push(Die::new("11110"));

    dont_cares.push(Die::new("10001"));
    dont_cares.push(Die::new("10010"));

    let mut table: ConesnsusTable<5> = ConesnsusTable::new(dice, dont_cares);
    table.solve();

    let compare: ConesnsusTable<5> = ConesnsusTable::from(vec![
        ConsensusTableEntry {
            num: Some(0),
            creators: None,
            die: Die::new("10001"),
            covered: Some(0),
            dont_care: true,
        },
        ConsensusTableEntry {
            num: Some(1),
            creators: None,
            die: Die::new("10010"),
            covered: Some(1),
            dont_care: true,
        },
        ConsensusTableEntry {
            num: Some(2),
            creators: None,
            die: Die::new("01100"),
            covered: Some(8),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(3),
            creators: None,
            die: Die::new("01101"),
            covered: Some(8),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(4),
            creators: None,
            die: Die::new("01110"),
            covered: Some(9),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(5),
            creators: None,
            die: Die::new("01111"),
            covered: Some(9),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(6),
            creators: None,
            die: Die::new("11101"),
            covered: Some(10),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(7),
            creators: None,
            die: Die::new("11110"),
            covered: Some(11),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(8),
            creators: Some([3, 2]),
            die: Die::new("0110-"),
            covered: Some(12),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(9),
            creators: Some([5, 4]),
            die: Die::new("0111-"),
            covered: Some(12),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(10),
            creators: Some([8, 6]),
            die: Die::new("-1101"),
            covered: None,
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(11),
            creators: Some([9, 7]),
            die: Die::new("-1110"),
            covered: None,
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(12),
            creators: Some([9, 8]),
            die: Die::new("011--"),
            covered: None,
            dont_care: false,
        },
    ]);

    assert!(compare.to_string() == table.to_string());
}

#[test]
pub fn test2() {
    let mut dice = Vec::new();

    dice.push(Die::new("01100"));
    dice.push(Die::new("01101"));
    dice.push(Die::new("01110"));
    dice.push(Die::new("01111"));
    dice.push(Die::new("11101"));
    dice.push(Die::new("11110"));

    let mut table: ConesnsusTable<5> = ConesnsusTable::new(dice, Vec::new());
    table.solve();

    let compare: ConesnsusTable<5> = ConesnsusTable::from(vec![
        ConsensusTableEntry {
            num: Some(0),
            creators: None,
            die: Die::new("01100"),
            covered: Some(6),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(1),
            creators: None,
            die: Die::new("01101"),
            covered: Some(6),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(2),
            creators: None,
            die: Die::new("01110"),
            covered: Some(7),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(3),
            creators: None,
            die: Die::new("01111"),
            covered: Some(7),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(4),
            creators: None,
            die: Die::new("11101"),
            covered: Some(8),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(5),
            creators: None,
            die: Die::new("11110"),
            covered: Some(9),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(6),
            creators: Some([1, 0]),
            die: Die::new("0110-"),
            covered: Some(10),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(7),
            creators: Some([3, 2]),
            die: Die::new("0111-"),
            covered: Some(10),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(8),
            creators: Some([6, 4]),
            die: Die::new("-1101"),
            covered: None,
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(9),
            creators: Some([7, 5]),
            die: Die::new("-1110"),
            covered: None,
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(10),
            creators: Some([7, 6]),
            die: Die::new("011--"),
            covered: None,
            dont_care: false,
        },
    ]);

    assert!(compare.to_string() == table.to_string());
}

#[test]
pub fn used_merged_dont_care() {
    let mut dice = Vec::new();
    let mut dont_cares = Vec::new();

    dice.push(Die::new("-1000"));
    dont_cares.push(Die::new("00000"));
    dont_cares.push(Die::new("10000"));

    let mut table: ConesnsusTable<5> = ConesnsusTable::new(dice, dont_cares);
    table.solve();

    let compare: ConesnsusTable<5> = ConesnsusTable::from(vec![
        ConsensusTableEntry {
            num: Some(0),
            creators: None,
            die: Die::new("00000"),
            covered: Some(3),
            dont_care: true,
        },
        ConsensusTableEntry {
            num: Some(1),
            creators: None,
            die: Die::new("10000"),
            covered: Some(3),
            dont_care: true,
        },
        ConsensusTableEntry {
            num: Some(2),
            creators: None,
            die: Die::new("-1000"),
            covered: Some(4),
            dont_care: false,
        },
        ConsensusTableEntry {
            num: Some(3),
            creators: Some([1, 0]),
            die: Die::new("-0000"),
            covered: Some(4),
            dont_care: true,
        },
        ConsensusTableEntry {
            num: Some(4),
            creators: Some([3, 2]),
            die: Die::new("--000"),
            covered: None,
            dont_care: false,
        },
    ]);

    assert!(compare.to_string() == table.to_string());
}
