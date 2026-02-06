import nox
from nox import Session


@nox.session(
    venv_backend="uv",
    python=["3.11", "3.12", "3.13", "3.14"],
    # reuse_venv=True,
)
def test(session: Session) -> None:
    # * sync the environment from the uv lockfile
    session.run(
        "uv",
        "sync",
        "--locked",
        "--active",
    )
    # * run the tests
    session.run(
        "python",
        "-m",
        "pytest",
    )
