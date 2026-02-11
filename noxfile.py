import nox
from nox import Session


@nox.session(
    venv_backend="uv",
    python=["3.11", "3.12", "3.13", "3.14"],
    reuse_venv=True,
)
def test(session: Session) -> None:
    session.env["UV_PYTHON"] = session.virtualenv.location
    session.env["UV_PROJECT_ENVIRONMENT"] = session.virtualenv.location

    # * sync the environment from the uv lockfile
    session.run(
        "uv",
        "sync",
        "--frozen",
    )
    # * run the tests
    session.run(
        "python",
        "-m",
        "pytest",
    )
