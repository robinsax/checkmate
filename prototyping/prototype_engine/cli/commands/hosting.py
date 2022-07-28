from typing import List

from ...game import IGame, HostGame, player_for_name
from ...server import IServer, ServerParameters, create_server

from ..bases import ICLIArgument
from ..arguments import IntCLIArgument

from .common import BaseCLICommand

class HostGameCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('host',)

    def arguments(self) -> List[ICLIArgument]:
        return (IntCLIArgument(),)

    def command(self, port: int) -> str:
        def new_game(white: str, black: str):
            game = HostGame.new((player_for_name(white), player_for_name(black)))
            game.start()
            self.cli.set_state('game', game)

        server = create_server(ServerParameters(
            port=port,
            get_game=lambda: self.cli.expect_state('game', IGame),
            new_game=new_game
        ))
        self.cli.set_state('server', server)
        server.start()

        return 'hosted on %s'%port

class DehostGameCommand(BaseCLICommand):
    
    def verbs(self) -> List[str]:
        return ('dehost',)

    def command(self) -> str:
        server = self.cli.expect_state('server', IServer)
        server.stop()

        self.cli.set_state('server', None)

        return 'hosting off'
