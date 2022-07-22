from typing import List

from ...game import GameImpl
from ...serialization import JSONSerializer

from ..arguments import FilePathCLIArgument
from ..bases import ICLIArgument

from .common import BaseCLICommand

class LoadGameCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('load',)

    def arguments(self) -> List[ICLIArgument]:
        return (FilePathCLIArgument(ext='.json'),)

    def command(self, file_path: str) -> str:
        serializer = JSONSerializer()

        with open(file_path, 'rb') as src_file:
            self.cli.set_game(serializer.deserialize(src_file.read(), GameImpl))

        return str(self.cli.game)
