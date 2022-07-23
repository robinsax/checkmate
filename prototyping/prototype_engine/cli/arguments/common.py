from typing import Generic

from ..common import CLIHolder
from ..bases import ICLIArgument, TArgumentValueType

class BaseCLIArgument(CLIHolder, Generic[TArgumentValueType], ICLIArgument[TArgumentValueType]):
    pass
