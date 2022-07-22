from typing import List, Callable

from ..bases import IBoardState, IPosition

from .common import BasePiece, find_moves_walks

class Queen(BasePiece):

    def names(self) -> List[str]:
        return ('♕', '♛')
 
    def moves(self, board: IBoardState) -> List[IPosition]:
        return find_moves_walks(self, (
            lambda pos: pos.up(),
            lambda pos: pos.down(),
            lambda pos: pos.left(),
            lambda pos: pos.right(),
            lambda pos: pos.up().left(),
            lambda pos: pos.up().right(),
            lambda pos: pos.down().left(),
            lambda pos: pos.down().right()
        ))
