from typing import List

from ...game import IGame, HostGame
from ...game.players import HeuristicRandom, Human
from ...serialization import JSONSerializer

from ..arguments import FilePathCLIArgument
from ..bases import ICLIArgument

from .common import BaseCLICommand

class NewGameCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('new',)

    def command(self) -> str:
        game = HostGame.new((HeuristicRandom(), Human()))
        game.start()
        self.cli.set_state('game', game)

        return str(game)

class LoadGameCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('load',)

    def arguments(self) -> List[ICLIArgument]:
        return (FilePathCLIArgument(ext='.json'),)

    def command(self, file_path: str) -> str:
        serializer = JSONSerializer()

        with open(file_path, 'rb') as src_file:
            self.cli.set_state('game', serializer.deserialize(src_file.read(), HostGame))

        return str(self.cli.game)

class SaveGameCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('save',)

    def arguments(self) -> List[ICLIArgument]:
        return (FilePathCLIArgument(ext='.json', create=True),)

    def command(self, file_path: str) -> str:
        serializer = JSONSerializer()

        with open(file_path, 'wb') as dest_file:
            dest_file.write(serializer.serialize(self.cli.expect_state('game', IGame)))

        return 'saved'
