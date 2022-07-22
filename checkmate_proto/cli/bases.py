from typing import Generic, TypeVar, List, Any

from ..game import IGame

TArgumentValueType = TypeVar('TArgumentValueType')

class ICLIArgument(Generic[TArgumentValueType]):

    def set_cli(self, cli: 'ICLI') -> None:
        raise NotImplementedError('ICLIArgument.set_cli(cli)')

    def parse(self, value: str) -> TArgumentValueType:
        raise NotImplementedError('ICLIArgument.parse(value)')

class ICLI:

    @property
    def game(self) -> IGame:
        raise NotImplementedError('ICLI.game')

    def set_game(self, game: IGame) -> None:
        raise NotImplementedError('ICLI.set_game(game)')

class ICLICommand:

    def set_cli(self, cli: ICLI) -> None:
        raise NotImplementedError('ICLICommand.set_cli(cli)')

    def verbs(self) -> List[str]:
        raise NotImplementedError('ICLICommand.verbs()')

    def arguments(self) -> List[ICLIArgument]:
        raise NotImplementedError('ICLICommand.arguments(game)')

    def command(self, *args: List[Any]) -> str:
        raise NotImplementedError('ICLICommand.command(game, *args)')
