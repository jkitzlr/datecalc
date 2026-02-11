import re
from datetime import date
from pathlib import Path

import pytest

from datecalc import BusdayConvention, BusinessCalendar


@pytest.fixture
def holidays() -> list[date]:
    return [
        date(2026, 1, 1),
        date(2026, 1, 19),
        date(2026, 2, 16),
        date(2026, 5, 25),
        date(2026, 6, 19),
        date(2026, 7, 3),
        date(2026, 9, 7),
        date(2026, 10, 12),
        date(2026, 11, 11),
        date(2026, 11, 26),
        date(2026, 12, 25),
    ]


@pytest.fixture
def calendar(holidays: list[date]) -> BusinessCalendar:
    return BusinessCalendar(holidays=holidays, weekmask="1111100")


def test_no_op_cal() -> None:
    cal = BusinessCalendar()
    assert cal.holidays == []
    assert cal.weekmask == "1111100"


def test_is_holiday(calendar: BusinessCalendar, holidays: list[date]) -> None:
    rslt = [calendar.is_holiday(dt) for dt in holidays]
    assert all(rslt)


@pytest.mark.parametrize(
    argnames=["dt", "rslt"],
    argvalues=[
        (date(2026, 2, 2), True),
        (date(2026, 2, 3), True),
        (date(2026, 2, 4), True),
        (date(2026, 2, 5), True),
        (date(2026, 2, 6), True),
        (date(2026, 2, 7), False),
        (date(2026, 2, 8), False),
    ],
)
def test_is_weekday(dt: date, rslt: bool, calendar: BusinessCalendar) -> None:
    assert calendar.is_weekday(dt) == rslt


@pytest.mark.parametrize(
    argnames=["dt", "rslt"],
    argvalues=[
        (date(2026, 2, 2), False),
        (date(2026, 2, 3), False),
        (date(2026, 2, 4), False),
        (date(2026, 2, 5), False),
        (date(2026, 2, 6), False),
        (date(2026, 2, 7), True),
        (date(2026, 2, 8), True),
    ],
)
def test_is_weekend(dt: date, rslt: bool, calendar: BusinessCalendar) -> None:
    assert calendar.is_weekend(dt) == rslt


@pytest.mark.parametrize(
    argnames=["dt", "rslt"],
    argvalues=[
        (date(2026, 2, 5), date(2026, 2, 6)),
        (date(2026, 2, 6), date(2026, 2, 9)),
        (date(2026, 11, 10), date(2026, 11, 12)),
        (date(2026, 2, 13), date(2026, 2, 17)),
    ],
)
def test_succ(dt: date, rslt: date, calendar: BusinessCalendar) -> None:
    assert calendar.succ(dt) == rslt


@pytest.mark.parametrize(
    argnames=["dt", "conv", "rslt"],
    argvalues=[
        (date(2026, 2, 7), BusdayConvention.Following, date(2026, 2, 9)),
        (date(2026, 2, 7), BusdayConvention.Preceding, date(2026, 2, 6)),
        (date(2026, 1, 31), BusdayConvention.ModifiedFollowing, date(2026, 1, 30)),
        (date(2026, 2, 1), BusdayConvention.ModifiedPreceding, date(2026, 2, 2)),
    ],
)
def test_adjust(
    dt: date, conv: BusdayConvention, rslt: date, calendar: BusinessCalendar
) -> None: ...


def test_bom_bus(calendar: BusinessCalendar) -> None:
    assert calendar.bom_bus(date(2026, 2, 6)) == date(2026, 2, 2)


def test_eom_bus(calendar: BusinessCalendar) -> None:
    assert calendar.eom_bus(date(2026, 2, 6)) == date(2026, 2, 27)


def test_from_json(json_cal_path: Path, holidays: list[date]) -> None:
    cal = BusinessCalendar.from_json(json_cal_path)
    assert cal.holidays == holidays
    assert cal.weekmask == "1111100"


def test_from_json_str(test_cal: str, holidays: list[date]) -> None:
    cal = BusinessCalendar.from_json_str(test_cal)
    assert cal.holidays == holidays
    assert cal.weekmask == "1111100"


def test_to_json_str(calendar: BusinessCalendar, test_cal: str) -> None:
    text = calendar.to_json_str()
    assert re.sub(r"[\s\t\n]", "", text) == calendar.to_json_str()
