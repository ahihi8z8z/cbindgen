from libc.stdint cimport int8_t, int16_t, int32_t, int64_t, intptr_t
from libc.stdint cimport uint8_t, uint16_t, uint32_t, uint64_t, uintptr_t
cdef extern from *:
  ctypedef bint bool
  ctypedef struct va_list

cdef extern from *:

  cdef enum:
    X # = 2,
    Y,
  ctypedef uint32_t C;

  cdef struct A:
    int32_t m0;

  cdef struct B:
    int32_t x;
    float y;

  cdef enum:
    Foo,
    Bar,
    Baz,
  ctypedef uint8_t F_Tag;

  cdef struct Bar_Body:
    F_Tag tag;
    uint8_t x;
    int16_t y;

  cdef union F:
    F_Tag tag;
    int16_t foo;
    Bar_Body bar;

  cdef enum:
    Hello,
    There,
    Everyone,
  ctypedef uint8_t H_Tag;

  cdef struct There_Body:
    uint8_t x;
    int16_t y;

  cdef struct H:
    H_Tag tag;
    int16_t hello;
    There_Body there;

  void root(A x, B y, C z, F f, H h);
