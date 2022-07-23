from typing import TYPE_CHECKING, Type, TypeVar, Union

if TYPE_CHECKING:
    from .input import SerializedInput

class ISerializable:

    def serialize(self) -> Union[str, int, dict, list]:
        raise NotImplementedError('ISerializable.serialize()')

    def deserialize(self, data: 'SerializedInput') -> None:
        raise NotImplementedError('ISerializeable.deserialize()')

TDeserializedType = TypeVar('TDeserializedType', bound=ISerializable)

class ISerializer:

    def serialize(self, obj: ISerializable) -> bytes:
        raise NotImplementedError('Serializer.serialize(obj)')

    def deserialize(self, data: bytes, dest: Type[TDeserializedType]) -> TDeserializedType:
        raise NotImplementedError('Serializer.deserialize(data)')
