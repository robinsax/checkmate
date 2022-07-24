from typing import TYPE_CHECKING, Generic, TypeVar, List, Any

if TYPE_CHECKING:
    from .cli import CLI

from ..game import IGame

TArgumentValueType = TypeVar('TArgumentValueType')

class ICLIArgument(Generic[TArgumentValueType]):

    def set_cli(self, cli: 'CLI') -> None:
        raise NotImplementedError()

    def parse(self, value: str) -> TArgumentValueType:
        raise NotImplementedError()

class ICLICommand:

    def set_cli(self, cli: 'CLI') -> None:
        raise NotImplementedError()

    def verbs(self) -> List[str]:
        raise NotImplementedError()

    def arguments(self) -> List[ICLIArgument]:
        raise NotImplementedError()

    def command(self, *args: List[Any]) -> str:
        raise NotImplementedError()
