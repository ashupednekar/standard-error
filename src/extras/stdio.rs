use std::io;
use crate::{StandardError, Interpolate};

impl From<io::Error> for StandardError {
    fn from(error: io::Error) -> Self {
        let code = match error.kind() {
            io::ErrorKind::NotFound => "ER-IO-NOTFOUND",
            io::ErrorKind::PermissionDenied => "ER-IO-PERMISSION",
            io::ErrorKind::ConnectionRefused => "ER-IO-CONNECTION",
            io::ErrorKind::ConnectionReset => "ER-IO-RESET",
            io::ErrorKind::ConnectionAborted => "ER-IO-ABORTED",
            io::ErrorKind::NotConnected => "ER-IO-NOTCONNECTED",
            io::ErrorKind::AddrInUse => "ER-IO-ADDRINUSE",
            io::ErrorKind::AddrNotAvailable => "ER-IO-ADDRNOTAVAILABLE",
            io::ErrorKind::BrokenPipe => "ER-IO-BROKENPIPE",
            io::ErrorKind::AlreadyExists => "ER-IO-ALREADYEXISTS",
            io::ErrorKind::WouldBlock => "ER-IO-WOULDBLOCK",
            io::ErrorKind::TimedOut => "ER-IO-TIMEDOUT",
            io::ErrorKind::Interrupted => "ER-IO-INTERRUPTED",
            io::ErrorKind::UnexpectedEof => "ER-IO-UNEXPECTEDEOF",
            _ => "ER-IO-UNKNOWN",
        };

        StandardError::new(code).interpolate_err(error.to_string())
    }
}
