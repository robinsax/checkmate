from typing import List

from ...serialization import JSONSerializer

from ..arguments import FilePathCLIArgument
from ..bases import ICLIArgument

from .common import BaseCLICommand

class SaveGameCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('save',)

    def arguments(self) -> List[ICLIArgument]:
        return (FilePathCLIArgument(ext='.json', create=True),)

    def command(self, file_path: str) -> str:
        serializer = JSONSerializer()

        with open(file_path, 'wb') as dest_file:
            dest_file.write(serializer.serialize(self.game))

        return 'saved'
