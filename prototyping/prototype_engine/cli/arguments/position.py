import re

from ...model import Position
from ...game import IGame

from ..exception import CLIInputError

from .common import BaseCLIArgument

class PositionCLIArgument(BaseCLIArgument[Position]):

    def parse(self, value: str) -> Position:
        match = re.match('^([a-z])([0-9])$', value)
        if not match:
            raise CLIInputError('%s: expected position'%value)

        position = Position(match.group(2), match.group(1))
        if not self.cli.expect_state('game', IGame).board.is_valid(position):
            raise CLIInputError('%s: invalid position'%value)

        return position
