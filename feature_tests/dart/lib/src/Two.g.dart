// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class Two implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  Two._(this._underlying, bool isOwned) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_Two_destroy));
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'Two_destroy')
// ignore: non_constant_identifier_names
external void _Two_destroy(ffi.Pointer<ffi.Void> self);
