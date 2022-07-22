from typing import List

from ...serialization import JSONSerializer
from ...model import IPiece

from ..arguments import PieceCLIArgument
from ..bases import ICLIArgument

from .common import BaseCLICommand

class DebugPieceCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('dbg_p',)

    def arguments(self) -> List[ICLIArgument]:
        return (PieceCLIArgument(),)

    def command(self, piece: IPiece) -> str:
        serializer = JSONSerializer()

        return '\n'.join((
            str(piece.color),
            str(serializer.serialize(piece))
        ))
