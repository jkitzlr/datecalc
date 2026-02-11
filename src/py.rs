use std::{path::PathBuf, str::FromStr};

use chrono::NaiveDate;
use pyo3::{
    exceptions::{
        PyEOFError, PyException, PyFileNotFoundError, PyIOError, PyValueError,
    },
    prelude::*,
    types::PyType,
};

use crate::{BusdayConvention, BusinessCalendar, serde::Error};

impl From<Error> for PyErr {
    fn from(value: crate::serde::Error) -> Self {
        match value {
            Error::Data => PyValueError::new_err("Malformed data"),
            Error::Eof => PyEOFError::new_err("Unepectedly reached end of file input"),
            Error::FileNotFound => {
                PyFileNotFoundError::new_err("Requested file doesn't exist")
            }
            Error::Io => PyIOError::new_err("An I/O error occurred parsing input file"),
            Error::Serde => {
                PyException::new_err("An unexpected de/serialization error occurred")
            }
            Error::Syntax => PyValueError::new_err("Bad input file syntax"),
        }
    }
}

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

    #[classmethod]
    #[pyo3(name = "from_json")]
    fn from_json_py(_cls: &Bound<'_, PyType>, path: PathBuf) -> PyResult<Self> {
        Self::from_json(path).map_err(PyErr::from)
    }

    #[classmethod]
    #[pyo3(name = "from_json_str")]
    fn from_json_str_py(_cls: &Bound<'_, PyType>, text: String) -> PyResult<Self> {
        Self::from_json_str(&text).map_err(PyErr::from)
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

    #[pyo3(
        name = "add_busdays",
        signature = (dt, days, conv = BusdayConvention::Preceding)
    )]
    fn add_busdays_py(
        &self,
        dt: NaiveDate,
        days: u32,
        conv: BusdayConvention,
    ) -> NaiveDate {
        self.add_busdays(dt, days, conv)
    }

    #[pyo3(
        name = "sub_busdays",
        signature = (dt, days, conv = BusdayConvention::Following)
    )]
    fn sub_busdays_py(
        &self,
        dt: NaiveDate,
        days: u32,
        conv: BusdayConvention,
    ) -> NaiveDate {
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

    #[pyo3(name = "to_json_str")]
    fn to_json_str_py(&self) -> PyResult<String> {
        self.to_json_str().map_err(PyErr::from)
    }
}
