use crate::data_type;

/// Time-zone information for [`DateTime`].
pub enum DateTimeKind {
  /// The time specified is in the Coordinated Universal Time (UTC) time zone.
  Utc,
  /// The time specified is in the local time zone.
  Local,
}

/// An date-time value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DateTime(pub(crate) data_type::DateTime);

impl DateTime {
  /// Number of 100 nanoseconds that have elapsed since 12:00:00, January 1, 0001.
  /// The value can represent time instants in a granularity of 100 nanoseconds
  /// until 23:59:59.9999999, December 31, 9999.
  pub fn ticks(&self) -> i64 {
    i64::from(self.0) >> 2
  }

  /// Provides the time-zone information.
  pub fn kind(&self) -> Option<DateTimeKind> {
    match i64::from(self.0) & 0b11 {
      1 => Some(DateTimeKind::Utc),
      2 => Some(DateTimeKind::Local),
      _ => None,
    }
  }
}
