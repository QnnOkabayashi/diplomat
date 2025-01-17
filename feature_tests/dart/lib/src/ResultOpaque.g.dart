// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class ResultOpaque implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  ResultOpaque._(this._underlying, bool isOwned) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_ResultOpaque_destroy));

  /// 
  ///
  /// Throws [ErrorEnum] on failure.
  factory ResultOpaque(int i) {
    final result = _ResultOpaque_new(i);
    if (!result.isOk) {
      throw ErrorEnum.values[result.union.err];
    }
    return ResultOpaque._(result.union.ok, true);
  }

  /// 
  ///
  /// Throws [ErrorEnum] on failure.
  factory ResultOpaque.failingFoo() {
    final result = _ResultOpaque_new_failing_foo();
    if (!result.isOk) {
      throw ErrorEnum.values[result.union.err];
    }
    return ResultOpaque._(result.union.ok, true);
  }

  /// 
  ///
  /// Throws [ErrorEnum] on failure.
  factory ResultOpaque.failingBar() {
    final result = _ResultOpaque_new_failing_bar();
    if (!result.isOk) {
      throw ErrorEnum.values[result.union.err];
    }
    return ResultOpaque._(result.union.ok, true);
  }

  /// 
  ///
  /// Throws [VoidError] on failure.
  factory ResultOpaque.failingUnit() {
    final result = _ResultOpaque_new_failing_unit();
    if (!result.isOk) {
      throw VoidError();
    }
    return ResultOpaque._(result.union.ok, true);
  }

  /// 
  ///
  /// Throws [ErrorStruct] on failure.
  factory ResultOpaque.failingStruct(int i) {
    final result = _ResultOpaque_new_failing_struct(i);
    if (!result.isOk) {
      throw ErrorStruct._(result.union.err);
    }
    return ResultOpaque._(result.union.ok, true);
  }

  /// 
  ///
  /// Throws [ResultOpaque] on failure.
  static void newInErr(int i) {
    final result = _ResultOpaque_new_in_err(i);
    if (!result.isOk) {
      throw ResultOpaque._(result.union.err, true);
    }
  }

  /// 
  ///
  /// Throws [VoidError] on failure.
  static int newInt(int i) {
    final result = _ResultOpaque_new_int(i);
    if (!result.isOk) {
      throw VoidError();
    }
    return result.union.ok;
  }

  /// 
  ///
  /// Throws [ResultOpaque] on failure.
  static ErrorEnum newInEnumErr(int i) {
    final result = _ResultOpaque_new_in_enum_err(i);
    if (!result.isOk) {
      throw ResultOpaque._(result.union.err, true);
    }
    return ErrorEnum.values[result.union.ok];
  }

  void assertInteger(int i) {
    _ResultOpaque_assert_integer(_underlying, i);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'ResultOpaque_destroy')
// ignore: non_constant_identifier_names
external void _ResultOpaque_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<_ResultOpaqueInt32 Function(ffi.Int32)>(isLeaf: true, symbol: 'ResultOpaque_new')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ResultOpaque_new(int i);

@ffi.Native<_ResultOpaqueInt32 Function()>(isLeaf: true, symbol: 'ResultOpaque_new_failing_foo')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ResultOpaque_new_failing_foo();

@ffi.Native<_ResultOpaqueInt32 Function()>(isLeaf: true, symbol: 'ResultOpaque_new_failing_bar')
// ignore: non_constant_identifier_names
external _ResultOpaqueInt32 _ResultOpaque_new_failing_bar();

@ffi.Native<_ResultOpaqueVoid Function()>(isLeaf: true, symbol: 'ResultOpaque_new_failing_unit')
// ignore: non_constant_identifier_names
external _ResultOpaqueVoid _ResultOpaque_new_failing_unit();

@ffi.Native<_ResultOpaqueErrorStructFfi Function(ffi.Int32)>(isLeaf: true, symbol: 'ResultOpaque_new_failing_struct')
// ignore: non_constant_identifier_names
external _ResultOpaqueErrorStructFfi _ResultOpaque_new_failing_struct(int i);

@ffi.Native<_ResultVoidOpaque Function(ffi.Int32)>(isLeaf: true, symbol: 'ResultOpaque_new_in_err')
// ignore: non_constant_identifier_names
external _ResultVoidOpaque _ResultOpaque_new_in_err(int i);

@ffi.Native<_ResultInt32Void Function(ffi.Int32)>(isLeaf: true, symbol: 'ResultOpaque_new_int')
// ignore: non_constant_identifier_names
external _ResultInt32Void _ResultOpaque_new_int(int i);

@ffi.Native<_ResultInt32Opaque Function(ffi.Int32)>(isLeaf: true, symbol: 'ResultOpaque_new_in_enum_err')
// ignore: non_constant_identifier_names
external _ResultInt32Opaque _ResultOpaque_new_in_enum_err(int i);

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Opaque>, ffi.Int32)>(isLeaf: true, symbol: 'ResultOpaque_assert_integer')
// ignore: non_constant_identifier_names
external void _ResultOpaque_assert_integer(ffi.Pointer<ffi.Opaque> self, int i);
