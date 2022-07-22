from ...model import IPosition

from ..exception import CLIInputError

from .common import BaseCLIArgument
from .position import PositionCLIArgument

class PieceCLIArgument(BaseCLIArgument[IPosition]):

    def parse(self, value: str) -> IPosition:
        pos_parser = PositionCLIArgument()
        pos_parser.set_cli(self.cli)
        position = pos_parser.parse(value)

        if not position.piece:
            raise CLIInputError('%s: no piece there'%value)

        return position.piece
