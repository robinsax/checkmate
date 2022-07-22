import re

from ...model import IPosition
from ...game import IGame

from ..exception import CLIInputError

from .common import BaseCLIArgument

class PositionCLIArgument(BaseCLIArgument[IPosition]):

    def parse(self, value: str) -> IPosition:
        match = re.match('^([a-z])([0-9])$', value)
        if not match:
            raise CLIInputError('%s: expected position'%value)

        position = self.game.board[(match.group(2), match.group(1))]
        if not position.is_valid():
            raise CLIInputError('%s: invalid position'%value)

        return position
