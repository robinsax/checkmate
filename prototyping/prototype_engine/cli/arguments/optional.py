from typing import TYPE_CHECKING, Generic

from ..bases import ICLIArgument, TArgumentValueType

if TYPE_CHECKING:
    from ..cli import CLI

from .common import BaseCLIArgument

class OptionalCLIArgument(Generic[TArgumentValueType], BaseCLIArgument[TArgumentValueType]):
    _inner: ICLIArgument

    def __init__(self, *, inner: ICLIArgument = None):
        super().__init__()

        self._inner = inner

    def set_cli(self, cli: 'CLI') -> None:
        self._inner.set_cli(cli)
        super().set_cli(cli)

    def parse(self, value: str) -> TArgumentValueType:
        if value == '<none>':
            return None

        return self._inner.parse(value)
