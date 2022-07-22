import json

from enum import Enum
from typing import Type, Any

from ..bases import ISerializer, ISerializable
from ..input import SerializedInputImpl
from ..exceptions import SerializeError

class JSONSerializer(ISerializer):

    @classmethod
    def create(cls) -> ISerializer:
        return cls()

    def _make_serialized(self, item: Any):
        if isinstance(item, Enum):
            return item.value

        if isinstance(item, ISerializable):
            return item.serialize()

        raise SerializeError(item)

    def serialize(self, obj: ISerializable) -> bytes:
        return bytes(
            json.dumps(obj, default=self._make_serialized),
            'utf-8'
        )

    def _make_deserialized(self, data: any, dest_typ: Type[ISerializable]):
        return SerializedInputImpl(None, '<input>', data).as_instance(dest_typ)

    def deserialize(self, data: bytes, dest_cls: Type[ISerializable]) -> ISerializable:
        return self._make_deserialized(json.loads(str(data, 'utf-8')), dest_cls)
