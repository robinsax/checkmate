from typing import List

from ..bases import IBoardState, IPosition

from .common import BasePiece, filter_moves_valid

class King(BasePiece):

    def names(self) -> List[str]:
        return ('♔', '♚')
 
    def moves(self, board: IBoardState) -> List[IPosition]:
        return filter_moves_valid(self, (
            self.position.up(),
            self.position.up().left(),
            self.position.up().right(),
            self.position.left(),
            self.position.right(),
            self.position.down(),
            self.position.down().left(),
            self.position.down().right()
        ))
