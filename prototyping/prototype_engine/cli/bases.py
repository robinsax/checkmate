from typing import TYPE_CHECKING, Generic, TypeVar, List, Any

if TYPE_CHECKING:
    from .cli import CLI

from ..game import IGame

TArgumentValueType = TypeVar('TArgumentValueType')

class ICLIArgument(Generic[TArgumentValueType]):

    def set_cli(self, cli: 'CLI') -> None:
        raise NotImplementedError('ICLIArgument.set_cli(cli)')

    def parse(self, value: str) -> TArgumentValueType:
        raise NotImplementedError('ICLIArgument.parse(value)')

class ICLICommand:

    def set_cli(self, cli: 'CLI') -> None:
        raise NotImplementedError('ICLICommand.set_cli(cli)')

    def verbs(self) -> List[str]:
        raise NotImplementedError('ICLICommand.verbs()')

    def arguments(self) -> List[ICLIArgument]:
        raise NotImplementedError('ICLICommand.arguments(game)')

    def command(self, *args: List[Any]) -> str:
        raise NotImplementedError('ICLICommand.command(game, *args)')
