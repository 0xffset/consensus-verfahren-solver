#[derive(Clone, Copy)]
pub struct SecondQuineTableCell {
    pub entry: bool,
    state: CellState,
}

impl Default for SecondQuineTableCell {
    fn default() -> Self {
        Self {
            entry: false,
            state: CellState::NA,
        }
    }
}

#[derive(Clone, Copy)]
pub enum CellState {
    NA,
    Prime,
    DominatingRow,
    DominatingColumn,
}
