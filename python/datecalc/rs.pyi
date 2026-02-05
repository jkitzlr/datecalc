from datetime import date
from typing import Self

class BusinessCalendar:
    """Business calendar. TBD."""

    def __init__(
        self: Self, holidays: list[date] | None = None, weekmask: str = "1111100"
    ) -> Self: ...
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
