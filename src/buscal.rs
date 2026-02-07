use std::{collections::HashSet, str::FromStr};

use chrono::{Datelike, NaiveDate};
use pyo3::prelude::*;

static WEEKDAYS: &[u8; 7] = &[64, 32, 16, 8, 4, 2, 1];

#[pyclass]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BusdayConvention {
    Following,
    Preceding,
    ModifiedFollowing,
    ModifiedPreceding,
    None,
}

#[pyclass]
#[derive(FromPyObject)]
pub struct BusinessCalendar {
    holidays: HashSet<NaiveDate>,
    weekmask: u8,
}

impl BusinessCalendar {
    // TODO: this should return an Option or Result
    pub fn new(holidays_: Option<impl IntoIterator<Item = NaiveDate>>, weekmask_: &str) -> Self {
        let holidays = match holidays_ {
            None => HashSet::new(),
            Some(iter) => iter.into_iter().collect::<HashSet<NaiveDate>>(),
        };
        let weekmask = u8::from_str_radix(weekmask_, 2).unwrap();
        Self { holidays, weekmask }
    }

    /// Check whether ``dt`` is a weekday.
    pub fn is_weekday(&self, dt: NaiveDate) -> bool {
        let dow = dt.weekday().num_days_from_monday() as usize;
        (WEEKDAYS[dow] & self.weekmask) != 0
    }

    /// Check whether ``dt`` is a weekend.
    pub fn is_weekend(&self, dt: NaiveDate) -> bool {
        !self.is_weekday(dt)
    }

    /// Check whether ``dt`` is a holiday.
    pub fn is_holiday(&self, dt: NaiveDate) -> bool {
        self.holidays.contains(&dt)
    }

    /// Check whether ``dt`` is a valid busday.
    pub fn is_busday(&self, dt: NaiveDate) -> bool {
        self.is_weekday(dt) & !self.is_holiday(dt)
    }

    /// Get the next successive busday
    pub fn succ(&self, dt: NaiveDate) -> NaiveDate {
        let mut tmp = dt.succ_opt().unwrap();
        while !self.is_busday(tmp) {
            tmp = tmp.succ_opt().unwrap();
        }
        tmp
    }

    /// Get the preceding successive business day
    pub fn pred(&self, dt: NaiveDate) -> NaiveDate {
        let mut tmp = dt.pred_opt().unwrap();
        while !self.is_busday(tmp) {
            tmp = tmp.pred_opt().unwrap();
        }
        tmp
    }

    /// Add ``days`` business days to ``dt``.
    pub fn add_busdays(&self, dt: NaiveDate, days: u32, conv: BusdayConvention) -> NaiveDate {
        let mut tmp = self.adjust(dt, conv);
        let mut cntr = 0u32;
        while cntr < days {
            tmp = self.succ(tmp);
            cntr += 1;
        }
        tmp
    }

    /// Subtract ``days`` business days to ``dt``.
    pub fn sub_busdays(&self, dt: NaiveDate, days: u32, conv: BusdayConvention) -> NaiveDate {
        let mut tmp = self.adjust(dt, conv);
        let mut cntr = 0u32;
        while cntr < days {
            tmp = self.pred(tmp);
            cntr += 1;
        }
        tmp
    }

    /// Adjust ``dt`` according to business day convention ``conv``.
    pub fn adjust(&self, dt: NaiveDate, conv: BusdayConvention) -> NaiveDate {
        match conv {
            BusdayConvention::Following => self.foll(dt),
            BusdayConvention::Preceding => self.prec(dt),
            BusdayConvention::ModifiedFollowing => self.modfoll(dt),
            BusdayConvention::ModifiedPreceding => self.modprec(dt),
            BusdayConvention::None => dt,
        }
    }

    /// Get the first business day of the month of ``dt``.
    pub fn bom_bus(&self, dt: NaiveDate) -> NaiveDate {
        self.adjust(dt.with_day(1).unwrap(), BusdayConvention::Following)
    }

    /// Get the first business day of the month of ``dt``.
    pub fn eom_bus(&self, dt: NaiveDate) -> NaiveDate {
        self.adjust(
            dt.with_day(dt.num_days_in_month() as u32).unwrap(),
            BusdayConvention::Preceding,
        )
    }

    // * -------------------------------------------------------------------------------
    // * PRIVATE METHODS
    // * -------------------------------------------------------------------------------

    fn foll(&self, dt: NaiveDate) -> NaiveDate {
        if self.is_busday(dt) {
            dt
        } else {
            self.succ(dt)
        }
    }

    fn prec(&self, dt: NaiveDate) -> NaiveDate {
        if self.is_busday(dt) {
            dt
        } else {
            self.pred(dt)
        }
    }

    fn modfoll(&self, dt: NaiveDate) -> NaiveDate {
        let tmp = self.foll(dt);
        if tmp.month() != dt.month() {
            self.prec(dt)
        } else {
            tmp
        }
    }

    fn modprec(&self, dt: NaiveDate) -> NaiveDate {
        let tmp = self.prec(dt);
        if tmp.month() != dt.month() {
            self.succ(dt)
        } else {
            tmp
        }
    }
}

// TODO: need to have code to return weekmask in different forms
#[pymethods]
impl BusinessCalendar {
    #[pyo3(signature = (holidays = None, weekmask = String::from_str("1111100").unwrap()))]
    #[new]
    fn new_py(holidays: Option<Vec<NaiveDate>>, weekmask: String) -> PyResult<Self> {
        let rslt = match holidays {
            None => Self::new(None::<Vec<NaiveDate>>, &weekmask),
            Some(h) => Self::new(Some(h.into_iter()), &weekmask),
        };
        Ok(rslt)
    }

    #[getter]
    fn holidays(&self) -> PyResult<Vec<NaiveDate>> {
        Ok(self.holidays.clone().into_iter().collect())
    }

    #[getter]
    fn weekmask(&self) -> PyResult<String> {
        let bstr = format!("{:b}", self.weekmask);
        Ok(bstr)
    }

    #[pyo3(name = "is_busday")]
    fn is_busday_py(&self, dt: NaiveDate) -> bool {
        self.is_busday(dt)
    }

    #[pyo3(name = "is_holiday")]
    fn is_holiday_py(&self, dt: NaiveDate) -> bool {
        self.is_holiday(dt)
    }

    #[pyo3(name = "is_weekday")]
    fn is_weekday_py(&self, dt: NaiveDate) -> bool {
        self.is_weekday(dt)
    }

    #[pyo3(name = "is_weekend")]
    fn is_weekend_py(&self, dt: NaiveDate) -> bool {
        self.is_weekend(dt)
    }

    #[pyo3(name = "succ")]
    fn succ_py(&self, dt: NaiveDate) -> NaiveDate {
        self.succ(dt)
    }

    #[pyo3(name = "pred")]
    fn pred_py(&self, dt: NaiveDate) -> NaiveDate {
        self.pred(dt)
    }

    #[pyo3(name = "add_busdays", signature = (dt, days, conv = BusdayConvention::Preceding))]
    fn add_busdays_py(&self, dt: NaiveDate, days: u32, conv: BusdayConvention) -> NaiveDate {
        self.add_busdays(dt, days, conv)
    }

    #[pyo3(name = "sub_busdays", signature = (dt, days, conv = BusdayConvention::Following))]
    fn sub_busdays_py(&self, dt: NaiveDate, days: u32, conv: BusdayConvention) -> NaiveDate {
        self.sub_busdays(dt, days, conv)
    }

    #[pyo3(name = "adjust")]
    fn adjust_py(&self, dt: NaiveDate, conv: BusdayConvention) -> NaiveDate {
        self.adjust(dt, conv)
    }

    #[pyo3(name = "bom_bus")]
    fn bom_bus_py(&self, dt: NaiveDate) -> NaiveDate {
        self.bom_bus(dt)
    }

    #[pyo3(name = "eom_bus")]
    fn eom_bus_py(&self, dt: NaiveDate) -> NaiveDate {
        self.eom_bus(dt)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Days, NaiveDate};

    use super::{BusdayConvention, BusinessCalendar};

    static HOLIDAYS: &[NaiveDate] = &[
        NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2026, 1, 19).unwrap(),
        NaiveDate::from_ymd_opt(2026, 2, 16).unwrap(),
        NaiveDate::from_ymd_opt(2026, 5, 25).unwrap(),
        NaiveDate::from_ymd_opt(2026, 6, 19).unwrap(),
        NaiveDate::from_ymd_opt(2026, 7, 3).unwrap(),
        NaiveDate::from_ymd_opt(2026, 9, 7).unwrap(),
        NaiveDate::from_ymd_opt(2026, 10, 12).unwrap(),
        NaiveDate::from_ymd_opt(2026, 11, 11).unwrap(),
        NaiveDate::from_ymd_opt(2026, 11, 26).unwrap(),
        NaiveDate::from_ymd_opt(2026, 12, 25).unwrap(),
    ];

    fn get_calendar() -> BusinessCalendar {
        BusinessCalendar::new(Some(HOLIDAYS.iter().cloned()), "1111100")
    }

    #[test]
    fn test_is_holiday() {
        let cal = get_calendar();
        let rslt = HOLIDAYS
            .iter()
            .map(|dt| cal.is_holiday(*dt))
            .reduce(|acc, e| acc & e)
            .unwrap();
        assert!(rslt)
    }

    #[test]
    fn test_is_weekday() {
        let cal = get_calendar();
        let dt = NaiveDate::from_ymd_opt(2026, 2, 2).unwrap();
        let dates = (0..6).map(|x| dt.checked_add_days(Days::new(x)).unwrap());
        let expected = [true, true, true, true, true, false, false];
        let rslt = dates
            .enumerate()
            .map(|(i, dt)| cal.is_weekday(dt) == expected[i])
            .reduce(|acc, e| acc & e)
            .unwrap();
        assert!(rslt)
    }

    #[test]
    fn test_is_weekend() {
        let cal = get_calendar();
        let dt = NaiveDate::from_ymd_opt(2026, 2, 2).unwrap();
        let dates = (0..6).map(|x| dt.checked_add_days(Days::new(x)).unwrap());
        let expected = [false, false, false, false, false, true, true];
        let rslt = dates
            .enumerate()
            .map(|(i, dt)| cal.is_weekend(dt) == expected[i])
            .reduce(|acc, e| acc & e)
            .unwrap();
        assert!(rslt)
    }

    #[test]
    fn test_succ() {
        let cal = get_calendar();
        // * next cal day is busday
        {
            let dt = NaiveDate::from_ymd_opt(2026, 2, 5).unwrap();
            let rslt = NaiveDate::from_ymd_opt(2026, 2, 6).unwrap();
            assert_eq!(cal.succ(dt), rslt);
        }
        // * next cal day is Saturday
        {
            let dt = NaiveDate::from_ymd_opt(2026, 2, 6).unwrap();
            let rslt = NaiveDate::from_ymd_opt(2026, 2, 9).unwrap();
            assert_eq!(cal.succ(dt), rslt);
        }
        // * next cal day is holiday
        {
            let dt = NaiveDate::from_ymd_opt(2026, 11, 10).unwrap();
            let rslt = NaiveDate::from_ymd_opt(2026, 11, 12).unwrap();
            assert_eq!(cal.succ(dt), rslt);
        }
        // * next cal day is saturday, Monday a holiday
        {
            let dt = NaiveDate::from_ymd_opt(2026, 2, 13).unwrap();
            let rslt = NaiveDate::from_ymd_opt(2026, 2, 17).unwrap();
            assert_eq!(cal.succ(dt), rslt);
        }
    }

    #[test]
    fn test_adjust() {
        let cal = get_calendar();
        // * test foll
        {
            let dt = NaiveDate::from_ymd_opt(2026, 2, 7).unwrap();
            let rslt = NaiveDate::from_ymd_opt(2026, 2, 9).unwrap();
            let conv = BusdayConvention::Following;
            assert_eq!(cal.adjust(dt, conv), rslt)
        }
        // * test preceding
        {
            let dt = NaiveDate::from_ymd_opt(2026, 2, 7).unwrap();
            let rslt = NaiveDate::from_ymd_opt(2026, 2, 6).unwrap();
            let conv = BusdayConvention::Preceding;
            assert_eq!(cal.adjust(dt, conv), rslt)
        }
        // * test modfoll
        {
            let dt = NaiveDate::from_ymd_opt(2026, 1, 31).unwrap();
            let rslt = NaiveDate::from_ymd_opt(2026, 1, 30).unwrap();
            let conv = BusdayConvention::ModifiedFollowing;
            assert_eq!(cal.adjust(dt, conv), rslt)
        }
        // * test modpreceding
        {
            let dt = NaiveDate::from_ymd_opt(2026, 2, 1).unwrap();
            let rslt = NaiveDate::from_ymd_opt(2026, 2, 2).unwrap();
            let conv = BusdayConvention::ModifiedPreceding;
            assert_eq!(cal.adjust(dt, conv), rslt)
        }
    }

    #[test]
    fn test_bom_bus() {
        let cal = get_calendar();
        let dt = NaiveDate::from_ymd_opt(2026, 2, 6).unwrap();
        assert_eq!(
            cal.bom_bus(dt),
            NaiveDate::from_ymd_opt(2026, 2, 2).unwrap()
        );
    }

    #[test]
    fn test_eom_bus() {
        let cal = get_calendar();
        let dt = NaiveDate::from_ymd_opt(2026, 2, 6).unwrap();
        assert_eq!(
            cal.eom_bus(dt),
            NaiveDate::from_ymd_opt(2026, 2, 27).unwrap()
        );
    }
}
