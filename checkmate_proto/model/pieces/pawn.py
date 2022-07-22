from typing import List

from ..bases import IBoardState, IPosition
from ..color import Color

from .common import BasePiece

class Pawn(BasePiece):

    def names(self) -> List[str]:
        return ('♙', '♟')

    def moves(self, board: IBoardState) -> List[IPosition]:
        moves = list()

        def forward(pos: IPosition) -> IPosition:
            if self.color is Color.white:
                return pos.up()
            return pos.down()

        walk_pos = forward(self.position)
        if walk_pos.is_valid() and not walk_pos.is_occupied():
            moves.append(walk_pos)

            if not len(self._prev_positions):
                walk_pos = forward(walk_pos)
                if walk_pos.is_valid() and not walk_pos.is_occupied():
                    moves.append(walk_pos)

        def check_attack(position: IPosition):
            valid_attack = (
                position.is_valid() and
                position.has_enemy(self)
            )
            if valid_attack:
                moves.append(position)

        check_attack(forward(self.position).right())
        check_attack(forward(self.position).left())

        return moves
