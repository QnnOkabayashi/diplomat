// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

final class One implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  One._(this._underlying, bool isOwned) {
    if (isOwned) {
      _finalizer.attach(this, _underlying.cast());
    }
  }

  static final _finalizer = ffi.NativeFinalizer(ffi.Native.addressOf(_One_destroy));

  factory One.transitivity(One hold, One nohold) {
    final result = _One_transitivity(hold._underlying, nohold._underlying);
    return One._(result, true);
  }

  factory One.cycle(Two hold, One nohold) {
    final result = _One_cycle(hold._underlying, nohold._underlying);
    return One._(result, true);
  }

  factory One.manyDependents(One a, One b, Two c, Two d, Two nohold) {
    final result = _One_many_dependents(a._underlying, b._underlying, c._underlying, d._underlying, nohold._underlying);
    return One._(result, true);
  }

  factory One.returnOutlivesParam(Two hold, One nohold) {
    final result = _One_return_outlives_param(hold._underlying, nohold._underlying);
    return One._(result, true);
  }

  factory One.diamondTop(One top, One left, One right, One bottom) {
    final result = _One_diamond_top(top._underlying, left._underlying, right._underlying, bottom._underlying);
    return One._(result, true);
  }

  factory One.diamondLeft(One top, One left, One right, One bottom) {
    final result = _One_diamond_left(top._underlying, left._underlying, right._underlying, bottom._underlying);
    return One._(result, true);
  }

  factory One.diamondRight(One top, One left, One right, One bottom) {
    final result = _One_diamond_right(top._underlying, left._underlying, right._underlying, bottom._underlying);
    return One._(result, true);
  }

  factory One.diamondBottom(One top, One left, One right, One bottom) {
    final result = _One_diamond_bottom(top._underlying, left._underlying, right._underlying, bottom._underlying);
    return One._(result, true);
  }

  factory One.diamondAndNestedTypes(One a, One b, One c, One d, One nohold) {
    final result = _One_diamond_and_nested_types(a._underlying, b._underlying, c._underlying, d._underlying, nohold._underlying);
    return One._(result, true);
  }

  factory One.implicitBounds(One explicitHold, One implicitHold, One nohold) {
    final result = _One_implicit_bounds(explicitHold._underlying, implicitHold._underlying, nohold._underlying);
    return One._(result, true);
  }

  factory One.implicitBoundsDeep(One explicit, One implicit1, One implicit2, One nohold) {
    final result = _One_implicit_bounds_deep(explicit._underlying, implicit1._underlying, implicit2._underlying, nohold._underlying);
    return One._(result, true);
  }
}

@ffi.Native<ffi.Void Function(ffi.Pointer<ffi.Void>)>(isLeaf: true, symbol: 'One_destroy')
// ignore: non_constant_identifier_names
external void _One_destroy(ffi.Pointer<ffi.Void> self);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_transitivity')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_transitivity(ffi.Pointer<ffi.Opaque> hold, ffi.Pointer<ffi.Opaque> nohold);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_cycle')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_cycle(ffi.Pointer<ffi.Opaque> hold, ffi.Pointer<ffi.Opaque> nohold);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_many_dependents')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_many_dependents(ffi.Pointer<ffi.Opaque> a, ffi.Pointer<ffi.Opaque> b, ffi.Pointer<ffi.Opaque> c, ffi.Pointer<ffi.Opaque> d, ffi.Pointer<ffi.Opaque> nohold);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_return_outlives_param')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_return_outlives_param(ffi.Pointer<ffi.Opaque> hold, ffi.Pointer<ffi.Opaque> nohold);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_top')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_top(ffi.Pointer<ffi.Opaque> top, ffi.Pointer<ffi.Opaque> left, ffi.Pointer<ffi.Opaque> right, ffi.Pointer<ffi.Opaque> bottom);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_left')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_left(ffi.Pointer<ffi.Opaque> top, ffi.Pointer<ffi.Opaque> left, ffi.Pointer<ffi.Opaque> right, ffi.Pointer<ffi.Opaque> bottom);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_right')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_right(ffi.Pointer<ffi.Opaque> top, ffi.Pointer<ffi.Opaque> left, ffi.Pointer<ffi.Opaque> right, ffi.Pointer<ffi.Opaque> bottom);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_bottom')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_bottom(ffi.Pointer<ffi.Opaque> top, ffi.Pointer<ffi.Opaque> left, ffi.Pointer<ffi.Opaque> right, ffi.Pointer<ffi.Opaque> bottom);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_diamond_and_nested_types')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_diamond_and_nested_types(ffi.Pointer<ffi.Opaque> a, ffi.Pointer<ffi.Opaque> b, ffi.Pointer<ffi.Opaque> c, ffi.Pointer<ffi.Opaque> d, ffi.Pointer<ffi.Opaque> nohold);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_implicit_bounds')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_implicit_bounds(ffi.Pointer<ffi.Opaque> explicitHold, ffi.Pointer<ffi.Opaque> implicitHold, ffi.Pointer<ffi.Opaque> nohold);

@ffi.Native<ffi.Pointer<ffi.Opaque> Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Opaque>)>(isLeaf: true, symbol: 'One_implicit_bounds_deep')
// ignore: non_constant_identifier_names
external ffi.Pointer<ffi.Opaque> _One_implicit_bounds_deep(ffi.Pointer<ffi.Opaque> explicit, ffi.Pointer<ffi.Opaque> implicit1, ffi.Pointer<ffi.Opaque> implicit2, ffi.Pointer<ffi.Opaque> nohold);
