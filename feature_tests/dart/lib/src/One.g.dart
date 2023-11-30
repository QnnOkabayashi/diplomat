// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class One implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  One._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('One_destroy'));

  factory One.transitivity(One hold, One nohold) {
    final result = _One_transitivity(hold._underlying, nohold._underlying);
    return One._(result);
  }

  // ignore: non_constant_identifier_names
  static final _One_transitivity =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_transitivity')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.cycle(Two hold, One nohold) {
    final result = _One_cycle(hold._underlying, nohold._underlying);
    return One._(result);
  }

  // ignore: non_constant_identifier_names
  static final _One_cycle =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_cycle')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.manyDependents(One a, One b, Two c, Two d, Two nohold) {
    final result = _One_many_dependents(a._underlying, b._underlying, c._underlying, d._underlying, nohold._underlying);
    return One._(result);
  }

  // ignore: non_constant_identifier_names
  static final _One_many_dependents =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_many_dependents')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.returnOutlivesParam(Two hold, One nohold) {
    final result = _One_return_outlives_param(hold._underlying, nohold._underlying);
    return One._(result);
  }

  // ignore: non_constant_identifier_names
  static final _One_return_outlives_param =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_return_outlives_param')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.diamondTop(One top, One left, One right, One bottom) {
    final result = _One_diamond_top(top._underlying, left._underlying, right._underlying, bottom._underlying);
    return One._(result);
  }

  // ignore: non_constant_identifier_names
  static final _One_diamond_top =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_diamond_top')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.diamondLeft(One top, One left, One right, One bottom) {
    final result = _One_diamond_left(top._underlying, left._underlying, right._underlying, bottom._underlying);
    return One._(result);
  }

  // ignore: non_constant_identifier_names
  static final _One_diamond_left =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_diamond_left')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.diamondRight(One top, One left, One right, One bottom) {
    final result = _One_diamond_right(top._underlying, left._underlying, right._underlying, bottom._underlying);
    return One._(result);
  }

  // ignore: non_constant_identifier_names
  static final _One_diamond_right =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_diamond_right')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.diamondBottom(One top, One left, One right, One bottom) {
    final result = _One_diamond_bottom(top._underlying, left._underlying, right._underlying, bottom._underlying);
    return One._(result);
  }

  // ignore: non_constant_identifier_names
  static final _One_diamond_bottom =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_diamond_bottom')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.diamondAndNestedTypes(One a, One b, One c, One d, One nohold) {
    final result = _One_diamond_and_nested_types(a._underlying, b._underlying, c._underlying, d._underlying, nohold._underlying);
    return One._(result);
  }

  // ignore: non_constant_identifier_names
  static final _One_diamond_and_nested_types =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_diamond_and_nested_types')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.implicitBounds(One explicitHold, One implicitHold, One nohold) {
    final result = _One_implicit_bounds(explicitHold._underlying, implicitHold._underlying, nohold._underlying);
    return One._(result);
  }

  // ignore: non_constant_identifier_names
  static final _One_implicit_bounds =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_implicit_bounds')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);

  factory One.implicitBoundsDeep(One explicit, One implicit1, One implicit2, One nohold) {
    final result = _One_implicit_bounds_deep(explicit._underlying, implicit1._underlying, implicit2._underlying, nohold._underlying);
    return One._(result);
  }

  // ignore: non_constant_identifier_names
  static final _One_implicit_bounds_deep =
    _capi<ffi.NativeFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>>('One_implicit_bounds_deep')
      .asFunction<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}