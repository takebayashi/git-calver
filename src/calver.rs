use std::fmt;

use chrono::prelude::Datelike;
use chrono::prelude::Local;
use chrono::DateTime;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct CalVer {
    year: u16,
    month: u8,
    revision: i8,
}

impl CalVer {
    pub fn new(revision: i8) -> CalVer {
        let t: DateTime<Local> = Local::now();
        CalVer {
            year: t.year() as u16,
            month: t.month() as u8,
            revision,
        }
    }

    pub fn zero() -> CalVer {
        CalVer {
            year: 0,
            month: 0,
            revision: 0,
        }
    }

    pub fn next_version(&self) -> CalVer {
        let t: DateTime<Local> = Local::now();
        self.next_version_at(t)
    }

    pub fn next_version_at(&self, date: DateTime<Local>) -> CalVer {
        let date_ver = format!("{}.{}", date.year() % 100, date.month());
        let mut i = 0;
        loop {
            let v = format!("{}.{}", date_ver, i)
                .calver()
                .unwrap_or_else(|| CalVer::new(i));
            if v > *self {
                return v;
            }
            i += 1;
        }
    }
}

impl fmt::Display for CalVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.year % 100, self.month, self.revision)
    }
}

pub trait ToCalVer {
    fn calver(&self) -> Option<CalVer>;
}

impl ToCalVer for String {
    fn calver(&self) -> Option<CalVer> {
        let mut chunk_iter = self.splitn(3, '.');
        let y = chunk_iter.next()?.parse::<u16>().ok()?;
        let y = if y >= 100 { y } else { y + 2000 };
        let m = chunk_iter.next()?.parse::<u8>().ok()?;
        let r = chunk_iter.next()?.parse::<i8>().ok()?;
        Some(CalVer {
            year: y,
            month: m,
            revision: r,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_to_calver() {
        assert_eq!(
            String::from("20.2.1").calver().unwrap(),
            CalVer {
                year: 2020,
                month: 2,
                revision: 1,
            }
        );
        assert_eq!(
            String::from("2020.2.1").calver().unwrap(),
            CalVer {
                year: 2020,
                month: 2,
                revision: 1,
            }
        );
        assert_eq!(String::from("v0.1").calver(), None);
    }

    #[test]
    fn next_version_at() {
        // same month
        assert_eq!(
            String::from("20.2.1").calver().unwrap().next_version_at(
                DateTime::parse_from_rfc3339("2020-02-01T00:00:00+09:00")
                    .unwrap()
                    .into()
            ),
            CalVer {
                year: 2020,
                month: 2,
                revision: 2,
            }
        );

        // next month
        assert_eq!(
            String::from("20.2.1").calver().unwrap().next_version_at(
                DateTime::parse_from_rfc3339("2020-03-01T00:00:00+09:00")
                    .unwrap()
                    .into()
            ),
            CalVer {
                year: 2020,
                month: 3,
                revision: 0,
            }
        );

        // next year
        assert_eq!(
            String::from("20.2.1").calver().unwrap().next_version_at(
                DateTime::parse_from_rfc3339("2021-02-01T00:00:00+09:00")
                    .unwrap()
                    .into()
            ),
            CalVer {
                year: 2021,
                month: 2,
                revision: 0,
            }
        );
    }
}
