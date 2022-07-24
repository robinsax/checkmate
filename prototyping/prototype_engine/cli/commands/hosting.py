from typing import List

from ...game import IGame
from ...server import IServer, create_server

from ..bases import ICLIArgument
from ..arguments import IntCLIArgument

from .common import BaseCLICommand

class HostGameCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('host',)

    def arguments(self) -> List[ICLIArgument]:
        return (IntCLIArgument(),)

    def command(self, port: int) -> str:
        game = self.cli.expect_state('game', IGame)

        server = create_server(game, port)
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
