import re

from typing import Any, Callable, List, Type
from enum import Enum

from .bases import ISerializable
from .exceptions import DeserializeError

class _Mode(Enum):
    unwrap_list = 'unwrap_list'

class SerializedInput:
    _parent: 'SerializedInput'
    _key: str
    _inner: Any
    _mode: _Mode

    def __init__(
        self,
        parent: 'SerializedInput', key: str, inner: Any,
        mode: _Mode = None
    ) -> None:
        self._parent = parent
        self._key = key
        self._inner = inner
        self._mode = mode

    def _child(self, key: str, inner: Any, mode: _Mode = None):
        return SerializedInput(self, key, inner, mode)

    def _raise_expected(self, message: str):
        key_path = list()
        cur = self
        while cur:
            key_path.append(cur._key)
            cur = cur._parent

        raise DeserializeError('%s: expected %s'%(''.join(reversed(key_path)), message))

    def dict_lookup(self, key: str, allow_empty: bool = False) -> 'SerializedInput':
        if self._mode is _Mode.unwrap_list:
            raise NotImplementedError()

        if not isinstance(self._inner, dict):
            self._raise_expected('an object')

        if allow_empty and not key in self._inner or self._inner[key] is None:
            return None

        return self._child('.%s'%key, self._inner.get(key))

    def list_lookup(self, index: int, allow_empty: bool = False, _safe: bool = False) -> 'SerializedInput':
        if not _safe and self._mode is _Mode.unwrap_list:
            raise NotImplementedError()

        if not isinstance(self._inner, list):
            self._raise_expected('an array')

        inner = None
        if len(self._inner) > index:
            inner = self._inner[index]
        elif allow_empty:
            return None

        return self._child('[%d]'%index, inner)

    def list_unwrap(self) -> 'SerializedInput':
        if self._mode is _Mode.unwrap_list:
            raise NotImplementedError()

        return self._child('', self._inner, _Mode.unwrap_list)

    def _do_list_unwrap(self, map_fn: Callable[[int], Any]) -> List[Any]:
        if not isinstance(self._inner, list):
            self._raise_expected('an array')

        result = list()
        for k in range(len(self._inner)):
            result.append(map_fn(k))

        return result

    def as_instance(self, Dest: Type[ISerializable]) -> Any:
        if self._mode is _Mode.unwrap_list:
            return self._do_list_unwrap(lambda k: self.list_lookup(k, _safe=True).as_instance(Dest))

        if Enum in Dest.__bases__:
            try:
                return getattr(Dest, self.as_str())
            except AttributeError:
                raise DeserializeError('invalid enum member: %s'%self._inner)

        attempt = Dest()
        attempt.deserialize(self)
        return attempt

    def as_str(self, pattern: str = None) -> str:
        if self._mode is _Mode.unwrap_list:
            return self._do_list_unwrap(lambda k: self.list_lookup(k, _safe=True).as_str(pattern))

        if not isinstance(self._inner, str):
            self._raise_expected('a string')

        if pattern and not re.match('^%s$'%pattern, self._inner):
            self._raise_expected('a string matching %s'%pattern)

        return self._inner

    def as_int(self) -> int:
        if self._mode is _Mode.unwrap_list:
            return self._do_list_unwrap(lambda k: self.list_lookup(k, True).as_int())

        if not isinstance(self._inner, int):
            self._raise_expected('an integer')

        return self._inner
