from typing import List

from ...game import IGame

from .common import BaseCLICommand

class ShowBoardCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('show', 'board')

    def command(self) -> str:
        return str(self.cli.expect_state('game', IGame))
