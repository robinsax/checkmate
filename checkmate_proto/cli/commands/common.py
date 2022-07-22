from typing import List

from ..common import CLIHolder
from ..bases import ICLICommand, ICLIArgument

class BaseCLICommand(CLIHolder, ICLICommand):
 
    def arguments(self) -> List[ICLIArgument]:
        return tuple()
