// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class _BorrowedFieldsReturningFfi extends ffi.Struct {
  external _SliceUtf8 bytes;
}

final class BorrowedFieldsReturning {
  String bytes;

  BorrowedFieldsReturning({required this.bytes});

  // ignore: unused_element
  BorrowedFieldsReturning._(_BorrowedFieldsReturningFfi underlying) :
    bytes = Utf8Decoder().convert(underlying.bytes._pointer.asTypedList(underlying.bytes._length));

  // ignore: unused_element
  _BorrowedFieldsReturningFfi _pointer(ffi.Allocator temp) {
    final pointer = temp<_BorrowedFieldsReturningFfi>();
    final bytesView = bytes.utf8View;
    pointer.ref.bytes._pointer = bytesView.pointer(temp);
    pointer.ref.bytes._length = bytesView.length;
    return pointer.ref;
  }

  @override
  bool operator ==(Object other) =>
      other is BorrowedFieldsReturning &&
      other.bytes == this.bytes;

  @override
  int get hashCode => Object.hashAll([
        this.bytes,
      ]);
}
