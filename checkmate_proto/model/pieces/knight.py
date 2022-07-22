from typing import List, Callable

from ..bases import IBoardState, IPosition

from .common import BasePiece, filter_moves_valid

class Knight(BasePiece):

    def names(self) -> List[str]:
        return ('♘', '♞')
 
    def moves(self, board: IBoardState) -> List[IPosition]:
        return filter_moves_valid(self, (
            self.position.up().up().left(),
            self.position.up().up().right(),
            self.position.down().down().left(),
            self.position.down().down().right(),
            self.position.left().left().up(),
            self.position.right().right().up(),
            self.position.left().left().down(),
            self.position.right().right().down()
        ))
