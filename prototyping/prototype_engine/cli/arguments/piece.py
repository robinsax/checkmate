from typing import Tuple

from ...model import Piece, Position

from ..exception import CLIInputError

from .common import BaseCLIArgument
from .position import PositionCLIArgument

class PieceCLIArgument(BaseCLIArgument[Tuple[Position, Piece]]):

    def parse(self, value: str) -> Piece:
        pos_parser = PositionCLIArgument()
        pos_parser.set_cli(self.cli)

        position = pos_parser.parse(value)
        piece = self.game.board[position]

        if not piece:
            raise CLIInputError('%s: no piece there'%value)

        return (position, piece)
