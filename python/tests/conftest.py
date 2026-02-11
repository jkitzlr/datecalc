from pathlib import Path

import pytest


@pytest.fixture
def json_cal_path() -> Path:
    return Path(__file__).parent / "data/test_cal.json"


@pytest.fixture
def test_cal(json_cal_path: Path) -> str:
    with json_cal_path.open("r") as fp:
        text = fp.read()

    return text
