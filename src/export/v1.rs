use crate::export::Exported;
use crate::export::VersionInfo;
use crate::export::VersionUint;
use crate::traits::Digit;
use crate::BigInt;
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

const BIG_UINT_ID: u8 = 0;
const BIG_INT_POSITIVE_ID: u8 = 1;
const BIG_INT_NEGATIVE_ID: u8 = 2;

/// Carries the header data of a file. New versions of this struct
/// might be created as versions change (keeping old ones)
#[derive(Debug, Copy, Clone)]
struct Header {
    // LINE 1: 16 bytes
    version: VersionUint, // 2 bytes
    integer: u8,          // 1 byte
    // LINE 2: 16 bytes
    lines: u32, // 4 bytes
}

impl VersionInfo<1> {
    fn read_header(file: &mut File) -> Result<Header> {
        let mut buff = [0u8; Self::LINE_SIZE_IN_BYTES];

        file.read_exact(&mut buff)?;
        let version = VersionUint::from_le_bytes(buff[..2].try_into().unwrap());
        if version != Self::VERSION {
            return Err(Error::new(ErrorKind::InvalidData, "Version not recognized"));
        }
        let integer = buff[2];

        file.read_exact(&mut buff)?;
        let lines = u32::from_le_bytes(buff[..4].try_into().unwrap());

        Ok(Header {
            version,
            integer,
            lines,
        })
    }

    fn write_header(file: &mut File, header: Header) -> Result<()> {
        let mut buff = [0u8; Self::LINE_SIZE_IN_BYTES];

        let version_bytes: [u8; 2] = header.version.to_le_bytes();
        file.write(&version_bytes)?;
        file.write(&[header.integer])?;
        file.write(&buff[3..])?;

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
        let uint = BigUint::from(digits);

        Ok(match header.integer {
            BIG_UINT_ID => Imported::Uint(uint),
            BIG_INT_POSITIVE_ID => Imported::Int(BigInt::from(uint)),
            BIG_INT_NEGATIVE_ID => Imported::Int(-BigInt::from(uint)),
            _ => return Err(Error::new(ErrorKind::InvalidData, "Integer not recognized")),
        })
    }

    fn export<T: Digit>(file: &mut File, exported: Exported<T>) -> Result<()> {
        let (mut header, digits) = match exported {
            Exported::Uint(uint) => {
                let header = Header {
                    version: Self::VERSION,
                    integer: BIG_UINT_ID,
                    lines: 0,
                };
                (header, &uint.val)
            }
            Exported::Int(int) => {
                let header = Header {
                    version: Self::VERSION,
                    integer: if int.sign {
                        BIG_INT_POSITIVE_ID
                    } else {
                        BIG_INT_NEGATIVE_ID
                    },
                    lines: 0,
                };
                (header, &int.uint.val)
            }
        };

        file.seek(SeekFrom::Start(0))?;

        // Write fake header
        Self::write_header(file, header)?;

        // Write digits
        let lines = Self::export_digits_to_binary_file(file, digits)?;

        // Write actual header
        file.seek(SeekFrom::Start(0))?;
        header.lines = lines
            .try_into()
            .expect("number of lines should not exceed a u32 in size !");
        Self::write_header(file, header)?;

        Ok(())
    }
}
