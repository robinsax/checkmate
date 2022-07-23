from typing import List, Tuple

from ...serialization import JSONSerializer
from ...model import Piece, Position

from ..arguments import PieceCLIArgument
from ..bases import ICLIArgument

from .common import BaseCLICommand

class DebugPieceCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('debug',)

    def arguments(self) -> List[ICLIArgument]:
        return (PieceCLIArgument(),)

    def command(self, piece: Tuple[Position, Piece]) -> str:
        serializer = JSONSerializer()

        return '\n'.join((
            str(piece[0]), str(piece[1].color)
        ))
