use crate::data_type;

/// A decimal number.
#[derive(Debug, Clone, PartialEq)]
pub struct Decimal(pub(crate) data_type::Decimal);
