
// /// Errors that can occur as a consequence of greedy input selection.
// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum GreedyInputSelectorError<ChangeStrategyErrT, NoteRefT> {
//     /// An intermediate value overflowed or underflowed the valid monetary range.
//     Balance(BalanceError),
//     /// A unified address did not contain a supported receiver.
//     UnsupportedAddress(Box<UnifiedAddress>),
//     /// An error was encountered in change selection.
//     Change(ChangeError<ChangeStrategyErrT, NoteRefT>),
// }
