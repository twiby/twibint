//! This module deals with exporting or importing one of the `twibint`
//! integers to files.

use crate::traits::Digit;
use crate::BigUint;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use std::io::Result;
use std::path::Path;

mod v1;

// To make a new version: increment TWIBINT_FILE_VERSION
// add enum variant to Version, and implement VersionInfoData on it
//
// Ideally the Imported and Exported enum should only grow, but not change

/// Current version
const TWIBINT_FILE_VERSION: VersionUint = 1;

/// the first 16 bits of every file must absolutely begin with this
type VersionUint = u16;
fn get_version(file: &mut File) -> Result<Version> {
    let mut buff = [0u8; 2];
    file.read_exact(&mut buff)?;
    file.seek(SeekFrom::Start(0))?;
    VersionUint::from_le_bytes(buff).try_into()
}

/// Empty struct meant to carry export/import implementations
struct VersionInfo<const VERSION: VersionUint>;

/// enum enabling choosing from a version at runtime
enum Version {
    V1(VersionInfo<1>),
}
impl TryFrom<VersionUint> for Version {
    type Error = Error;

    fn try_from(value: VersionUint) -> Result<Version> {
        match value {
            1 => Ok(Version::V1(VersionInfo::<1>)),
            _ => Err(Error::new(ErrorKind::InvalidData, "Version not recognized")),
        }
    }
}

trait VersionInfoData {
    const LINE_SIZE_IN_BYTES: usize;
    const VERSION: u16;
    fn export_digits_to_binary_file<T: Digit>(file: &mut File, digits: &[T]) -> Result<usize> {
        let bytes_per_digit = T::NB_BITS / 8;
        let digits_per_buffer = Self::LINE_SIZE_IN_BYTES / bytes_per_digit;
        debug_assert_eq!(T::NB_BITS % 8, 0);
        debug_assert_eq!(Self::LINE_SIZE_IN_BYTES % bytes_per_digit, 0);

        let mut lines = 0;
        let mut buff = vec![0u8; Self::LINE_SIZE_IN_BYTES];
        for chunk in digits.chunks(digits_per_buffer) {
            buff.fill(0);

            for (d, sub_buff) in chunk.into_iter().zip(buff.chunks_mut(bytes_per_digit)) {
                d.write_bytes(sub_buff);
            }

            file.write(&buff)?;
            lines += 1;
        }

        Ok(lines)
    }

    fn import_binary_file_to_digits<T: Digit>(file: &mut File, lines: usize) -> Result<Vec<T>> {
        let bytes_per_digit = T::NB_BITS / 8;
        let digits_per_buffer = Self::LINE_SIZE_IN_BYTES / bytes_per_digit;
        debug_assert_eq!(T::NB_BITS % 8, 0);
        debug_assert_eq!(Self::LINE_SIZE_IN_BYTES % bytes_per_digit, 0);

        let mut digits = Vec::<T>::with_capacity(lines * digits_per_buffer);
        let mut buff = vec![0u8; Self::LINE_SIZE_IN_BYTES];
        for _ in 0..lines {
            file.read_exact(&mut buff)?;
            for sub_buff in buff.chunks(bytes_per_digit) {
                digits.push(T::read_bytes(sub_buff));
            }
        }

        Ok(digits)
    }

    fn import<T: Digit>(self, file: &mut File) -> Result<Imported<T>>;
    fn export<T: Digit>(file: &mut File, exported: Exported<T>) -> Result<()>;
}

enum Exported<'a, T: Digit> {
    Uint(&'a BigUint<T>),
}

pub enum Imported<T: Digit> {
    Uint(BigUint<T>),
}

impl<T: Digit> Imported<T> {
    /// This should only be used on files generated by `twibint`
    pub fn read_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut file = File::open(path)?;
        match get_version(&mut file)? {
            Version::V1(v) => v.import(&mut file),
        }
    }
}

impl<T: Digit> BigUint<T> {
    pub fn write_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = File::create(path)?;
        VersionInfo::<TWIBINT_FILE_VERSION>::export(&mut file, Exported::Uint(self))
    }
}

#[cfg(test)]
fn export_digits_to_binary_file<T: Digit>(file: &mut File, digits: &[T]) -> Result<usize> {
    VersionInfo::<TWIBINT_FILE_VERSION>::export_digits_to_binary_file(file, digits)
}

#[cfg(test)]
fn import_binary_file_to_digits<T: Digit>(file: &mut File, lines: usize) -> Result<Vec<T>> {
    VersionInfo::<TWIBINT_FILE_VERSION>::import_binary_file_to_digits(file, lines)
}

#[cfg(test)]
mod tests {
    use super::export_digits_to_binary_file;
    use super::import_binary_file_to_digits;
    use super::Imported;
    use super::VersionInfo;
    use super::VersionInfoData;
    use super::TWIBINT_FILE_VERSION;
    use crate::traits::Digit;
    use crate::BigUint;
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    use typed_test_gen::test_with;

    fn file_name<T: Digit>(n: &str) -> String {
        let mut name = "test_file_".to_string();
        name.push_str(n);
        name.push_str(&T::NB_BITS.to_string());
        name.push_str(".txt");
        name
    }

    fn create_file<T: Digit>(n: &str) -> File {
        let name = file_name::<T>(n);
        File::create(name).unwrap()
    }

    fn open_file<T: Digit>(n: &str) -> File {
        let name = file_name::<T>(n);
        File::open(name).unwrap()
    }

    #[test]
    fn write_file() {
        {
            let mut file = File::create("test_file_write_file.txt").unwrap();
            file.write(b"Hello, world!").unwrap();
        }

        {
            let mut file = File::open("test_file_write_file.txt").unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            assert_eq!(contents, "Hello, world!");
        }
    }

    #[test_with(u32, u64)]
    fn write_ones<T: Digit>() {
        const BYTES_TO_WRITE: usize = 30;
        let n = (BigUint::<T>::from(1u32) << (BYTES_TO_WRITE * 8)) - T::ONE;

        {
            let mut file = create_file::<T>("write_ones");
            let lines = export_digits_to_binary_file(&mut file, &n.val).unwrap();
            assert_eq!(lines, 2);
        }

        {
            let mut file = open_file::<T>("write_ones");
            let mut buff = Vec::<u8>::new();
            file.read_to_end(&mut buff).unwrap();
            let mut should_get = vec![255u8; BYTES_TO_WRITE];
            while should_get.len() % VersionInfo::<TWIBINT_FILE_VERSION>::LINE_SIZE_IN_BYTES != 0 {
                should_get.push(0);
            }
            assert_eq!(buff, should_get);
        }

        {
            let mut file = open_file::<T>("write_ones");
            let digits = import_binary_file_to_digits::<T>(&mut file, 2).unwrap();
            let n2 = BigUint::from(digits);
            assert_eq!(n, n2);
        }
    }

    #[test_with(u32, u64)]
    fn write_ones_full<T: Digit>() {
        const BYTES_TO_WRITE: usize = 30;
        let n = (BigUint::<T>::from(1u32) << (BYTES_TO_WRITE * 8)) - T::ONE;

        let name = file_name::<T>("write_ones_full");
        n.write_to_file(&name).unwrap();

        match Imported::<T>::read_from_file(name).unwrap() {
            Imported::Uint(n2) => assert_eq!(n, n2),
        }
    }

    #[test_with(u32, u64)]
    fn read_fact_500<T: Digit>() {
        let mut n = BigUint::<T>::from(1u32);
        for m in 2..500u32 {
            n *= BigUint::<T>::from(m);
        }

        for v in 1..TWIBINT_FILE_VERSION + 1 {
            let mut name = "src/export/test_files/fact_500_v".to_string();
            name.push_str(&v.to_string());
            name.push_str(".tw");
            match Imported::<T>::read_from_file(name).unwrap() {
                Imported::Uint(n2) => assert_eq!(n, n2),
            }
        }
    }

    #[test_with(u32, u64)]
    fn read_sqrt_2<T: Digit>() {
        for v in 1..TWIBINT_FILE_VERSION + 1 {
            let mut name = "src/export/test_files/sqrt_2_v".to_string();
            name.push_str(&v.to_string());
            name.push_str(".tw");
            let n = match Imported::<T>::read_from_file(name).unwrap() {
                Imported::Uint(n) => n,
            };

            let next_pow_2 = BigUint::<T>::from(1u32) << n.nb_bits();
            assert!(n < next_pow_2);
            assert!((&n + T::ONE) * (&n + T::ONE) > next_pow_2);
        }
    }
}