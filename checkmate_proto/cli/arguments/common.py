from typing import Generic

from ..common import CLIHolder
from ..bases import ICLI, ICLIArgument, TArgumentValueType

class BaseCLIArgument(CLIHolder, Generic[TArgumentValueType], ICLIArgument[TArgumentValueType]):
    pass
