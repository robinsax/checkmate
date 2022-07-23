from typing import List

from ...game import HostGame
from ...game.players import HumanCLI, HeuristicRandom

from .common import BaseCLICommand

class NewGameCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('new',)

    def command(self) -> str:
        self.cli.set_game(HostGame.new((HeuristicRandom(), HumanCLI())))

        return str(self.cli.game)
