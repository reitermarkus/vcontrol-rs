//! 2.7 Binary Record Grammar

mod classes;
pub use classes::{Class, Classes};
mod null_object;
pub use null_object::NullObject;
mod method_call;
pub use method_call::{CallArray, MethodCall};
mod method_return;
pub use method_return::{MethodReturn, ReturnCallArray};
mod remoting_message;
pub use remoting_message::{MethodCallOrReturn, RemotingMessage};
