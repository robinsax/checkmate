import sys

from typing import List

from ...server import IServer

from ..exception import CLIInputError

from .common import BaseCLICommand

class ExitCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('exit', 'bye')

    def command(self) -> str:
        if self.cli.get_state('server', IServer):
            raise CLIInputError('actively hosting')

        sys.exit(1)
