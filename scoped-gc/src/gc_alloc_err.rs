/// Represents an allocation error
///
/// This is currently unused but may be returned once more checks are implemented around the
/// garbage-collector.
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum GcAllocErr {
  /// Signals that the garbage collector exhausted all its available memory.
  Exhausted,
}
