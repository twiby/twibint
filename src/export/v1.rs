use crate::export::Exported;
use crate::export::VersionInfo;
use crate::export::VersionUint;
use crate::traits::Digit;
use crate::BigUint;
use crate::Imported;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use crate::export::VersionInfoData;

/// Carries the header data of a file. New versions of this struct
/// might be created as versions change (keeping old ones)

#[derive(Debug, Copy, Clone)]
struct Header {
    // LINE 1
    version: VersionUint,
    // LINE 2
    lines: u32,
}

impl VersionInfo<1> {
    fn read_header(file: &mut File) -> Result<Header> {
        let mut buff = [0u8; Self::LINE_SIZE_IN_BYTES];

        file.read_exact(&mut buff)?;
        let version = VersionUint::from_le_bytes(buff[..2].try_into().unwrap());
        if version != Self::VERSION {
            return Err(Error::new(ErrorKind::InvalidData, "Version not recognized"));
        }

        file.read_exact(&mut buff)?;
        let lines = u32::from_le_bytes(buff[..4].try_into().unwrap());

        Ok(Header { version, lines })
    }

    fn write_header(file: &mut File, header: Header) -> Result<()> {
        let mut buff = [0u8; Self::LINE_SIZE_IN_BYTES];

        let version_bytes: [u8; 2] = header.version.to_le_bytes();
        file.write(&version_bytes)?;
        file.write(&buff[2..])?;

        buff.fill(0);
        let lines_bytes: [u8; 4] = header.lines.to_le_bytes();
        file.write(&lines_bytes)?;
        file.write(&buff[4..])?;

        Ok(())
    }
}

impl VersionInfoData for VersionInfo<1> {
    const LINE_SIZE_IN_BYTES: usize = 16;
    const VERSION: VersionUint = 1;

    fn import<T: Digit>(self, file: &mut File) -> Result<Imported<T>> {
        let header = Self::read_header(file)?;
        let digits = Self::import_binary_file_to_digits(file, header.lines.try_into().unwrap())?;
        Ok(Imported::Uint(BigUint::from(digits)))
    }

    fn export<T: Digit>(file: &mut File, exported: Exported<T>) -> Result<()> {
        match exported {
            Exported::Uint(uint) => {
                file.seek(SeekFrom::Start(0))?;

                // Write fake header
                let mut header = Header {
                    version: Self::VERSION,
                    lines: 0,
                };
                Self::write_header(file, header)?;

                // Write digits
                let lines = Self::export_digits_to_binary_file(file, &uint.val)?;

                // Write actual header
                file.seek(SeekFrom::Start(0))?;
                header.lines = lines
                    .try_into()
                    .expect("number of lines should not exceed a u32 in size !");
                Self::write_header(file, header)?;

                Ok(())
            }
        }
    }
}
