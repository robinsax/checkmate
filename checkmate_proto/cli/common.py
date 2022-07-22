from ..game import IGame

from .bases import ICLI
from .exception import CLIInputError

class CLIHolder:
    _cli: ICLI

    def set_cli(self, cli: ICLI) -> None:
        self._cli = cli

    @property
    def cli(self) -> ICLI:
        return self._cli

    @property
    def game(self) -> IGame:
        if not self._cli.game:
            raise CLIInputError('no active game')

        return self._cli.game
