from typing import List

from ...game import create_default_game

from .common import BaseCLICommand

class NewGameCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('new',)

    def command(self) -> str:
        self.cli.set_game(create_default_game())

        return str(self.cli.game)
