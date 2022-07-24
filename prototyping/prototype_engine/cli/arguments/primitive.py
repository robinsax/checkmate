import re

from ..exception import CLIInputError

from .common import BaseCLIArgument

class StringCLIArgument(BaseCLIArgument[str]):
    _pattern: str

    def __init__(self, pattern: str = None) -> None:
        super().__init__()

        self._pattern = pattern

    def parse(self, value: str) -> str:
        if self._pattern and not re.match('^%s$'%self._pattern, value):
            raise CLIInputError('%s: invalid format'%value)

        return value

class IntCLIArgument(BaseCLIArgument[int]):

    def parse(self, value: str) -> str:
        try:
            return int(value)
        except:
            raise CLIInputError('%s: expected number'%value)

