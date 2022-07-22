import os

from ..exception import CLIInputError

from .common import BaseCLIArgument

class FilePathCLIArgument(BaseCLIArgument[str]):
    _create: bool
    _ext: str

    def __init__(self, *, ext: str = '.out', create: bool = False) -> None:
        super().__init__()

        self._ext = ext
        self._create = create

    def parse(self, value: str) -> str:
        value = ''.join((value, self._ext))

        check_path = os.path.dirname(value) if self._create else value

        if not os.path.exists(check_path):
            raise CLIInputError('%s: invalid file'%value)

        return value
