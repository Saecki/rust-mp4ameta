use std::io::{Read, Seek, SeekFrom};
use std::time::Duration;

use super::*;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Mvhd {
    /// The duration of the track.
    pub duration: Duration,
}

impl ParseAtom for Mvhd {
    const FOURCC: Fourcc = MOVIE_HEADER;

    fn parse_atom(reader: &mut (impl Read + Seek), len: u64) -> crate::Result<Self> {
        let mut mvhd = Self::default();
        let start = reader.seek(SeekFrom::Current(0))?;

        let (version, _) = parse_full_head(reader)?;
        match version {
            0 => {
                // # Version 0
                // 1 byte version
                // 3 bytes flags
                // 4 bytes creation time
                // 4 bytes motification time
                // 4 bytes time scale
                // 4 bytes duration
                // ...
                reader.seek(SeekFrom::Current(8))?;
                let timescale = data::read_u32(reader)? as u64;
                let duration = data::read_u32(reader)? as u64;

                mvhd.duration = Duration::from_nanos(duration * 1_000_000_000 / timescale);
            }
            1 => {
                // # Version 1
                // 1 byte version
                // 3 bytes flags
                // 8 bytes creation time
                // 8 bytes motification time
                // 4 bytes time scale
                // 8 bytes duration
                // ...
                reader.seek(SeekFrom::Current(16))?;
                let timescale = read_u32(reader)? as u64;
                let duration = read_u64(reader)?;

                mvhd.duration = Duration::from_nanos(duration * 1_000_000_000 / timescale);
            }
            v => {
                return Err(crate::Error::new(
                    crate::ErrorKind::UnknownVersion(version),
                    format!("Error unknown movie header (mvhd) version {}", v),
                ))
            }
        }

        data::seek_to_end(reader, start, len)?;

        Ok(mvhd)
    }
}