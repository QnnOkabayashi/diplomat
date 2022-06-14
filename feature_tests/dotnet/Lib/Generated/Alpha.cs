// Automatically generated by Diplomat

#pragma warning disable 0105
using System;
using System.Runtime.InteropServices;

using DiplomatFeatures.Diplomat;
#pragma warning restore 0105

namespace DiplomatFeatures;

#nullable enable

public partial class Alpha
{
    private Raw.Alpha _inner;

    /// <summary>
    /// Creates a managed <c>Alpha</c> from the raw representation.
    /// </summary>
    public unsafe Alpha(Raw.Alpha data)
    {
        _inner = data;
    }

    /// <summary>
    /// Returns a copy of the underlying raw representation.
    /// </summary>
    public Raw.Alpha AsFFI()
    {
        return _inner;
    }
}
