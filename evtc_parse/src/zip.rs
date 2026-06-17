use crate::{Log, Parse, ParseError};
use evtc::Event;
use std::io;
use zip::{ZipArchive, result::ZipError};

/// Parses a [`Log`] from a compressed `zevtc` input.
pub fn parse_zevtc(input: impl io::Read + io::Seek) -> Result<Log, ParseError> {
    Log::parse_zevtc(input)
}

impl<T> Log<T> {
    /// Parses a [`Log`] from a compressed `zevtc` input.
    pub fn parse_zevtc(input: impl io::Read + io::Seek) -> Result<Self, ParseError>
    where
        T: From<Event>,
    {
        let mut archive = ZipArchive::new(input).expect("input log file not compressed");
        let mut file = archive.by_index(0).expect("input log file empty");
        Self::parse(&mut file)
    }
}

impl From<ZipError> for ParseError {
    fn from(err: ZipError) -> Self {
        match err {
            ZipError::Io(io) => Self::IoError(io),
            _ => Self::NotEvtc,
        }
    }
}
