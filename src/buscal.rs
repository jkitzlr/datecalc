use std::{collections::HashSet, str::FromStr};

use chrono::{Datelike, NaiveDate};
use pyo3::prelude::*;

static WEEKDAYS: &[u8; 7] = &[64, 32, 16, 8, 4, 2, 1];

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

    /// Check whether ``dt`` is a holiday.
    pub fn is_holiday(&self, dt: NaiveDate) -> bool {
        self.holidays.contains(&dt)
    }

    /// Check whether ``dt`` is a valid busday.
    pub fn is_busday(&self, dt: NaiveDate) -> bool {
        self.is_weekday(dt) & !self.is_holiday(dt)
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
        self.is_busday(dt)
    }
}
