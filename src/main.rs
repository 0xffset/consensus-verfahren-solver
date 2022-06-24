use consensus_method::solve_using_consensus_method;
use die::{Die, DieSide};

mod consensus_method;
mod die;
mod table_entry;

fn main() {
    let mut dice = Vec::new();
    let mut dont_cares = Vec::new();

    // TODO: add that combinations of dont cares that arent used (like bellow) will be discarded

    dice.push(Die {
        content: [
            DieSide::Zero,
            DieSide::One,
            DieSide::One,
            DieSide::Zero,
            DieSide::Zero,
        ],
    });

    dont_cares.push(Die {
        content: [
            DieSide::One,
            DieSide::Zero,
            DieSide::Zero,
            DieSide::Zero,
            DieSide::Zero,
        ],
    });
    dont_cares.push(Die {
        content: [
            DieSide::Zero,
            DieSide::Zero,
            DieSide::Zero,
            DieSide::Zero,
            DieSide::Zero,
        ],
    });

    solve_using_consensus_method(dice, dont_cares);
}
