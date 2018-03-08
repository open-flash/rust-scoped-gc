#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug)]
pub enum GcAllocErr {
  Exhausted,
}
