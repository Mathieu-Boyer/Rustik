use crate::globals::Move;
pub enum SearchReturnValues {
    SolutionFound(Vec<Move>),
    ThresholdExceeded(usize),
    NoSolution
}