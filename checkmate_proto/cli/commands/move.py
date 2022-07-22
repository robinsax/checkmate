from typing import List

from ...game import GameStateError
from ...model import IPiece, IPosition

from ..bases import ICLIArgument
from ..arguments import PieceCLIArgument, PositionCLIArgument
from ..exception import CLIInputError

from .common import BaseCLICommand

class MoveCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('move',)

    def arguments(self) -> List[ICLIArgument]:
        return (PieceCLIArgument(), PositionCLIArgument())

    def command(self, piece: IPiece, position: IPosition) -> str:
        try:
            self.game.take_turn(piece, position)
        except GameStateError as err:
            raise CLIInputError('nope, %s'%str(err))

        return str(self.game)
