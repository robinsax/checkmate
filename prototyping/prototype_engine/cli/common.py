from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from .cli import CLI

class CLIHolder:
    _cli: 'CLI'

    def set_cli(self, cli: 'CLI') -> None:
        self._cli = cli

    @property
    def cli(self) -> 'CLI':
        return self._cli
