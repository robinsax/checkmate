import re

from typing import Any, Callable, List, Tuple, Type
from enum import Enum

from .bases import ISerializedInput, ISerializable
from .exceptions import DeserializeError

class _Mode(Enum):
    unwrap_list = 'unwrap_list'

class SerializedInputImpl(ISerializedInput):
    _parent: 'SerializedInputImpl'
    _key: str
    _inner: Any
    _mode: _Mode

    def __init__(
        self,
        parent: 'SerializedInputImpl', key: str, inner: Any,
        mode: _Mode = None
    ) -> None:
        self._parent = parent
        self._key = key
        self._inner = inner
        self._mode = mode

    def _child(self, key: str, inner: Any, mode: _Mode = None):
        return SerializedInputImpl(self, key, inner, mode)

    def _raise_expected(self, message: str):
        key_path = list()
        cur = self
        while cur:
            key_path.append(cur._key)
            cur = cur._parent

        raise DeserializeError('%s: expected %s'%(''.join(reversed(key_path)), message))

    def dict_lookup(self, key: str) -> ISerializedInput:
        if self._mode is _Mode.unwrap_list:
            raise NotImplementedError()

        if not isinstance(self._inner, dict):
            self._raise_expected('an object')

        return self._child('.%s'%key, self._inner.get(key))

    def list_lookup(self, index: int, _safe: bool = False) -> ISerializedInput:
        if not _safe and self._mode is _Mode.unwrap_list:
            raise NotImplementedError()

        if not isinstance(self._inner, list):
            self._raise_expected('an array')

        inner = None
        if len(self._inner) > index:
            inner = self._inner[index]

        return self._child('[%d]'%index, inner)

    def list_unwrap(self) -> ISerializedInput:
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

    def as_instance(self, *types: List[Type[ISerializable]]) -> List[Any]:
        if self._mode is _Mode.unwrap_list:
            return self._do_list_unwrap(lambda k: self.list_lookup(k, True).as_instance(*types))

        errs = list()
        for typ in types:
            try:
                attempt = typ()
                attempt.deserialize(self)
                return attempt
            except DeserializeError as err:
                errs.append(err)

        message, errs = str(errs[0]), errs[1:]
        prev_expected = message.split('expected ')[1]
        for err in errs:
            expected = str(err).split('expected ')[1]
            this_expected = expected

            for k in range(min(len(expected), len(prev_expected))):
                if expected[k] != prev_expected[k]:
                    expected = expected[k:]
                    break
            else:
                continue

            prev_expected = this_expected
            message = ' or '.join((message, expected))
        raise DeserializeError(message)

    def as_str(self, pattern: str = None) -> str:
        if self._mode is _Mode.unwrap_list:
            return self._do_list_unwrap(lambda k: self.list_lookup(k, True).as_str(pattern))

        if not isinstance(self._inner, str):
            self._raise_expected('a string')

        if pattern and not re.match('^%s$'%pattern, self._inner):
            self._raise_expected('a string matching %s'%pattern)

        return self._inner

    def scan_str(self, pattern: str) -> Tuple[str, 'ISerializedInput']:
        if self._mode is _Mode.unwrap_list:
            raise NotImplementedError()

        if not isinstance(self._inner, str):
            self._raise_expected('a string')

        match = re.match('^(%s)(.*?)$'%pattern, self._inner)
        if not match:
            self._raise_expected('a string starting with %s'%pattern)

        scanned, rest = match.groups()
        return (scanned, self._child('[%d:]'%len(scanned), rest))

    def as_int(self) -> int:
        if self._mode is _Mode.unwrap_list:
            return self._do_list_unwrap(lambda k: self.list_lookup(k, True).as_int())

        if not isinstance(self._inner, int):
            self._raise_expected('an integer')

        return self._inner
