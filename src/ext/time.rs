use super::*;

pub trait TimeExt: Sized {
    fn now() -> Self;

    fn to_system_time(&self) -> std::time::SystemTime;

    fn try_from_system_time(time: std::time::SystemTime) -> Result<Self, jiff::Error>;

    #[cfg(feature = "time")]
    fn to_utc_date_time(&self) -> ::time::UtcDateTime;

    #[cfg(feature = "time")]
    fn try_from_utc_date_time(time: ::time::UtcDateTime) -> Result<Self, jiff::Error>;

    fn from_zoned(time: jiff::Zoned) -> Self;

    fn try_to_zoned(&self) -> Result<jiff::Zoned, jiff::Error> {
        let system_time = self.to_system_time();
        jiff::Zoned::try_from(system_time)
    }
}

impl TimeExt for metav1::Time {
    fn now() -> Self {
        Self(jiff::Timestamp::now())
    }

    fn to_system_time(&self) -> std::time::SystemTime {
        self.0.into()
    }

    fn try_from_system_time(time: std::time::SystemTime) -> Result<Self, jiff::Error> {
        time.try_into().map(Self)
    }

    fn from_zoned(time: jiff::Zoned) -> Self {
        Self(time.into())
    }

    #[cfg(feature = "time")]
    fn to_utc_date_time(&self) -> ::time::UtcDateTime {
        self.to_system_time().into()
    }

    #[cfg(feature = "time")]
    fn try_from_utc_date_time(time: ::time::UtcDateTime) -> Result<Self, jiff::Error> {
        Self::try_from_system_time(time.into())
    }
}
