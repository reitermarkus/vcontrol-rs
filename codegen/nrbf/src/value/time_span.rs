use crate::data_type;

/// A time span.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TimeSpan(pub(crate) data_type::TimeSpan);

impl TimeSpan {
  /// Duration as the number of 100 nanoseconds. The values range from -10675199 days, 2 hours, 48 minutes, and 05.4775808
  /// seconds to 10675199 days, 2 hours, 48 minutes, and 05.4775807 seconds inclusive.
  pub fn value(&self) -> i64 {
    self.0.into()
  }
}
