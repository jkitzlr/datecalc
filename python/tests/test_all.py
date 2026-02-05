from datetime import date

import pytest

from datecalc import BusinessCalendar


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
        date(2026, 11, 12),
        date(2026, 11, 11),
        date(2026, 11, 26),
        date(2026, 12, 25),
    ]


@pytest.fixture
def calendar(holidays: list[date]) -> BusinessCalendar:
    return BusinessCalendar(holidays=holidays, weekmask="1111100")


def test_is_holiday(calendar: BusinessCalendar, holidays: list[date]) -> None:
    rslt = [calendar.is_holiday(dt) for dt in holidays]
    assert all(rslt)


def test_no_op_cal() -> None:
    cal = BusinessCalendar()
    assert cal.holidays == []
    assert cal.weekmask == "1111100"
