from typing import Type, TypeVar, List, Union, Tuple, Any

class ISerializedInput:

    def dict_lookup(self, key: str) -> 'ISerializedInput':
        raise NotImplementedError('ISerializedInput.dict_lookup(key)')

    def list_lookup(self, index: int) -> 'ISerializedInput':
        raise NotImplementedError('ISerializedInput.list_lookup(index)')

    def list_unwrap(self) -> 'ISerializedInput':
        raise NotImplementedError('ISerializedInput.list_unwrap(map_fn?)')

    def as_instance(self, *types: List[Type['ISerializable']]) -> List[Any]:
        raise NotImplementedError('ISerializedInput.as_instance(*types)')

    def as_str(self, pattern: str = None) -> str:
        raise NotImplementedError('ISerializedInput.as_str(pattern?)')

    def scan_str(self, pattern: str) -> Tuple[str, 'ISerializedInput']:
        raise NotImplementedError('ISerializedInput.scan_str(pattern)')

    def as_int(self) -> int:
        raise NotImplementedError('ISerializedInput.as_int()')

class ISerializable:

    def serialize(self) -> Union[str, int, dict, list]:
        raise NotImplementedError('ISerializable.serialize()')

    def deserialize(self, data: ISerializedInput) -> None:
        raise NotImplementedError('ISerializeable.deserialize()')

TDeserializedType = TypeVar('TDeserializedType', bound=ISerializable)

class ISerializer:

    def serialize(self, obj: ISerializable) -> bytes:
        raise NotImplementedError('Serializer.serialize(obj)')

    def deserialize(self, data: bytes, dest: Type[TDeserializedType]) -> TDeserializedType:
        raise NotImplementedError('Serializer.deserialize(data)')
