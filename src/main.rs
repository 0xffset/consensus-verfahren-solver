use consensus_method::solve_using_consensus_method;
use die::{Die, DieSide};

mod consensus_method;
mod die;
mod table_entry;

fn main() {
    let mut dice = Vec::new();
    let mut dont_cares = Vec::new();

    dice.push(Die {
        content: [
            DieSide::Zero,
            DieSide::One,
            DieSide::One,
            DieSide::Zero,
            DieSide::Zero,
        ],
    });
    dice.push(Die {
        content: [
            DieSide::Zero,
            DieSide::One,
            DieSide::One,
            DieSide::Zero,
            DieSide::One,
        ],
    });
    dice.push(Die {
        content: [
            DieSide::Zero,
            DieSide::One,
            DieSide::One,
            DieSide::One,
            DieSide::Zero,
        ],
    });
    dice.push(Die {
        content: [
            DieSide::Zero,
            DieSide::One,
            DieSide::One,
            DieSide::One,
            DieSide::One,
        ],
    });
    dice.push(Die {
        content: [
            DieSide::One,
            DieSide::One,
            DieSide::One,
            DieSide::Zero,
            DieSide::One,
        ],
    });
    dice.push(Die {
        content: [
            DieSide::One,
            DieSide::One,
            DieSide::One,
            DieSide::One,
            DieSide::Zero,
        ],
    });

    dont_cares.push(Die {
        content: [
            DieSide::One,
            DieSide::Zero,
            DieSide::Zero,
            DieSide::Zero,
            DieSide::One,
        ],
    });
    dont_cares.push(Die {
        content: [
            DieSide::One,
            DieSide::Zero,
            DieSide::Zero,
            DieSide::One,
            DieSide::Zero,
        ],
    });

    solve_using_consensus_method(dice, dont_cares);
}
