from typing import TYPE_CHECKING

from ..game import IGame

from .exception import CLIInputError

if TYPE_CHECKING:
    from .cli import CLI

class CLIHolder:
    _cli: 'CLI'

    def set_cli(self, cli: 'CLI') -> None:
        self._cli = cli

    @property
    def cli(self) -> 'CLI':
        return self._cli

    @property
    def game(self) -> IGame:
        if not self._cli.game:
            raise CLIInputError('no active game')

        return self._cli.game
