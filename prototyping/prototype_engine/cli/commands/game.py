from typing import List

from ...game import IGame, HostGame, InvalidPlayerError, player_for_name
from ...serialization import JSONSerializer

from ..arguments import FilePathCLIArgument, OptionalCLIArgument, StringCLIArgument
from ..bases import ICLIArgument
from ..exception import CLIInputError

from .common import BaseCLICommand

class NewGameCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('new',)

    def arguments(self) -> List[ICLIArgument]:
        return (
            OptionalCLIArgument(inner=StringCLIArgument()),
            OptionalCLIArgument(inner=StringCLIArgument())
        )

    def command(self, white: str, black: str) -> str:
        game = HostGame.new()

        try:
            game.set_players((player_for_name(white), player_for_name(black)))
        except InvalidPlayerError as err:
            raise CLIInputError('invalid player: %s'%err)

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

        game: IGame = None
        with open(file_path, 'rb') as src_file:
            game = serializer.deserialize(src_file.read(), HostGame)

        game.set_players((player_for_name('human'), player_for_name('human')))
        game.start()
        self.cli.set_state('game', game)

        return str(game)

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
