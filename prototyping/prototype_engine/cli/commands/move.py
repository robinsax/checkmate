from typing import List, Tuple

from ...game import GameStateError
from ...game.players import HumanCLI
from ...model import Piece, Position

from ..bases import ICLIArgument
from ..arguments import PieceCLIArgument, PositionCLIArgument
from ..exception import CLIInputError

from .common import BaseCLICommand

class MoveCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('move',)

    def arguments(self) -> List[ICLIArgument]:
        return (PieceCLIArgument(), PositionCLIArgument())

    def command(self, piece_info: Tuple[Position, Piece], position: Position) -> str:
        if not isinstance(self.game.active_player, HumanCLI):
            raise CLIInputError('not your turn')

        try:
            self.game.make_move(self.game.as_move(piece_info[1], position))
        except GameStateError as err:
            raise CLIInputError('nope, %s'%str(err))

        return str(self.game)
