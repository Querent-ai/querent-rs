use log::SetLoggerError;
use std::{
	any::Any,
	backtrace::Backtrace,
	collections::HashMap,
	fmt,
	fmt::{Debug, Formatter},
	num::ParseIntError,
};
use tokio::{sync::mpsc::error::SendError, time::error::Elapsed};

#[derive(thiserror::Error, Debug)]
pub struct QuerentError {
	pub message: String,
	pub cause: QuerentErrorCauseType,
	pub backtrace: Option<Backtrace>,
}

#[derive(Debug, Clone)]
pub enum QuerentErrorCauseType {
	User(Option<HashMap<String, String>>),
	Internal(Option<HashMap<String, String>>),
}

impl QuerentError {
	pub fn user(message: String) -> Self {
		Self {
			message,
			cause: QuerentErrorCauseType::User(None),
			backtrace: Some(Backtrace::capture()),
		}
	}

	pub fn internal(message: String) -> Self {
		Self {
			message,
			cause: QuerentErrorCauseType::Internal(None),
			backtrace: Some(Backtrace::capture()),
		}
	}

	pub fn internal_with_bt(message: String, backtrace: Option<Backtrace>) -> Self {
		Self { message, cause: QuerentErrorCauseType::Internal(None), backtrace }
	}

	pub fn panic(error: Box<dyn Any + Send>) -> Self {
		if let Some(reason) = error.downcast_ref::<&str>() {
			QuerentError::internal(format!("Unexpected panic. Reason: {}", reason))
		} else if let Some(reason) = error.downcast_ref::<String>() {
			QuerentError::internal(format!("Unexpected panic. Reason: {}", reason))
		} else {
			QuerentError::internal("Unexpected panic without reason".to_string())
		}
	}
}

impl QuerentError {
	pub fn backtrace(&self) -> Option<&Backtrace> {
		self.backtrace.as_ref()
	}

	pub fn to_backtrace(self) -> Option<Backtrace> {
		self.backtrace
	}
}

impl fmt::Display for QuerentError {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self.cause {
			QuerentErrorCauseType::User(_) => f.write_fmt(format_args!("User: {}", self.message)),
			QuerentErrorCauseType::Internal(_) =>
				f.write_fmt(format_args!("Internal: {}", self.message)),
		}
	}
}

impl From<std::io::Error> for QuerentError {
	fn from(v: std::io::Error) -> Self {
		QuerentError::internal(v.to_string())
	}
}

impl From<tokio::task::JoinError> for QuerentError {
	fn from(v: tokio::task::JoinError) -> Self {
		if v.is_panic() {
			QuerentError::panic(v.into_panic())
		} else {
			// JoinError can return CanceledError
			QuerentError::internal(v.to_string())
		}
	}
}

impl<T> From<SendError<T>> for QuerentError
where
	T: Debug,
{
	fn from(v: SendError<T>) -> Self {
		QuerentError::internal(v.to_string())
	}
}

impl From<std::time::SystemTimeError> for QuerentError {
	fn from(v: std::time::SystemTimeError) -> Self {
		QuerentError::internal(v.to_string())
	}
}

impl From<Elapsed> for QuerentError {
	fn from(v: Elapsed) -> Self {
		QuerentError::internal(v.to_string())
	}
}

impl From<tokio::sync::broadcast::error::RecvError> for QuerentError {
	fn from(v: tokio::sync::broadcast::error::RecvError) -> Self {
		QuerentError::internal(v.to_string())
	}
}

impl From<std::string::FromUtf8Error> for QuerentError {
	fn from(v: std::string::FromUtf8Error) -> Self {
		QuerentError::internal(v.to_string())
	}
}

impl From<tokio::sync::oneshot::error::RecvError> for QuerentError {
	fn from(v: tokio::sync::oneshot::error::RecvError) -> Self {
		QuerentError::internal(v.to_string())
	}
}

impl From<tokio::sync::watch::error::SendError<bool>> for QuerentError {
	fn from(v: tokio::sync::watch::error::SendError<bool>) -> Self {
		QuerentError::internal(v.to_string())
	}
}

impl From<tokio::sync::watch::error::RecvError> for QuerentError {
	fn from(v: tokio::sync::watch::error::RecvError) -> Self {
		QuerentError::internal(v.to_string())
	}
}
impl From<ParseIntError> for QuerentError {
	fn from(v: ParseIntError) -> Self {
		QuerentError::internal(v.to_string())
	}
}

impl From<SetLoggerError> for QuerentError {
	fn from(v: SetLoggerError) -> Self {
		QuerentError::internal(v.to_string())
	}
}

impl From<serde_json::Error> for QuerentError {
	fn from(v: serde_json::Error) -> Self {
		QuerentError::internal(v.to_string())
	}
}

impl From<std::num::ParseFloatError> for QuerentError {
	fn from(v: std::num::ParseFloatError) -> Self {
		QuerentError::internal(v.to_string())
	}
}

impl From<base64::DecodeError> for QuerentError {
	fn from(v: base64::DecodeError) -> Self {
		QuerentError::internal(v.to_string())
	}
}

impl From<tokio::sync::AcquireError> for QuerentError {
	fn from(v: tokio::sync::AcquireError) -> Self {
		QuerentError::internal(v.to_string())
	}
}
