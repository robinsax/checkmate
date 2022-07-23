import sys

from typing import List

from .common import BaseCLICommand

class ExitCommand(BaseCLICommand):

    def verbs(self) -> List[str]:
        return ('exit', 'bye')

    def command(self) -> str:
        sys.exit(1)
