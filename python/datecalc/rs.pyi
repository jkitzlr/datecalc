from datetime import date
from enum import Enum
from typing import Any, Self

class BusdayConvention(Enum):
    """Conventions used to adjust non-business days to valid business days:
    - Following (next good business day)
    - Preceding (last good business day)
    - Modified Following (next good business day unless is in a new month--then last)
    - Modified Preceding (last good business day unless is in a diff month--then next)
    - None (don't adjust)
    """

    Following: Any
    Preceding: Any
    ModifiedFollowing: Any
    ModifiedPreceding: Any

class BusinessCalendar:
    """Business calendar. TBD."""

    def __init__(
        self: Self, holidays: list[date] | None = None, weekmask: str = "1111100"
    ) -> Self:
        """Initialize new BusinessCalendar instance.

        Args:
            holidays: List of holidays for this calendar. Defaults to None.
            weekmask: Bitstring representing weekdays Mon-Sun (1=weekday, 0=weekend).
                Defaults to "1111100".
        """

    @property
    def holidays(self: Self) -> list[date]:
        """Get the holidays associated with the calendar."""

    @property
    def weekmask(self: Self) -> str:
        """Get the bitstring representing whether each day of the week is a workday."""

    def is_weekday(self: Self, dt: date) -> bool:
        """Check whether ``dt`` is a weekday.

        Args:
            dt: The date to check.

        Returns:
            Whether ``dt`` is a weekday.
        """

    def is_weekend(self: Self, dt: date) -> bool:
        """Check whether ``dt`` is a weekend day.

        Args:
            dt: The date to check.

        Returns:
            Whether ``dt`` is a weekend day.
        """

    def is_holiday(self: Self, dt: date) -> bool:
        """Check whether ``dt`` is a holiday.

        Args:
            dt: The date to check.

        Returns:
            Whether ``dt`` is a holiday.
        """

    def is_busday(self: Self, dt: date) -> bool:
        """Check whether ``dt`` is a busday, i.e. a weekday that's not a holiday.

        Args:
            dt: The date to check.

        Returns:
            Whether ``dt`` is a business day.
        """

    def succ(self: Self, dt: date) -> date:
        """Get the next (successive) business date after ``dt``.

        Args:
            dt: Input date.

        Returns:
            Succeeding business date.
        """

    def pred(self: Self, dt: date) -> date:
        """Get the previous (predecessor) business date after ``dt``.

        Args:
            dt: Input date.

        Returns:
            Previous business date.
        """

    def add_busdays(self: Self, dt: date, days: int, conv: BusdayConvention) -> date:
        """Add ``days`` business days to ``dt`` after adjusting ``dt`` using ``conv``.

        Args:
            dt: The input date.
            days: Business days to add.
            conv: Adjustment to use to roll ``dt`` before adding the busdays.

        Returns:
            Result.
        """

    def sub_busdays(self: Self, dt: date, days: int, conv: BusdayConvention) -> date:
        """Subtract ``days`` business days from ``dt`` after adjusting ``dt`` using
        ``conv``.

        Args:
            dt: The input date.
            days: Business days to subtract.
            conv: Adjustment to use to roll ``dt`` before subtracting the busdays.

        Returns:
            Result.
        """

    def adjust(self: Self, dt: date, conv: BusdayConvention) -> date:
        """Adjust ``dt`` based on ``conv`` if ``dt`` not a valid busdate.

        Args:
            dt: The input date.
            conv: Adjustment to use to roll ``dt`` to good busday.

        Returns:
            Result.
        """

    def bom_bus(self: Self, dt: date) -> date:
        """Get the first business day of the month containing ``dt``.

        Args:
            dt: The input date.

        Returns:
            The first busday of the month.
        """

    def eom_bus(self: Self, dt: date) -> date:
        """Get the last business day of the month containing ``dt``.

        Args:
            dt: The input date.

        Returns:
            The last busday of the month.
        """
