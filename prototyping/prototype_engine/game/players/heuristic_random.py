from random import choice

from ...model import Move

from ..bases import IPlayer, IGame

class HeuristicRandom(IPlayer):

    def take_turn(self, game: IGame) -> None:
        moves = game.board.legal_moves(game.turn)

        game.make_move(choice(moves))
