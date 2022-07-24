from random import choice

from ...model import Move

from ..bases import IPlayer, IGame

class HeuristicRandom(IPlayer):

    def name(self) -> str:
        return 'heur_rand'

    def is_human(self) -> bool:
        return False

    def take_turn(self, game: IGame) -> None:
        moves = game.board.legal_moves(game.turn)

        game.make_move(choice(moves))
