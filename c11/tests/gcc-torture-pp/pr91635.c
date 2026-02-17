# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr91635.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr91635.c" 2




unsigned short b, c;
int u, v, w, x;

__attribute__ ((noipa)) int
foo (unsigned short c)
{
  c <<= __builtin_add_overflow (-c, -1, &b);
  c >>= 1;
  return c;
}

__attribute__ ((noipa)) int
bar (unsigned short b)
{
  b <<= -14 & 15;
  b = b >> -~1;
  return b;
}

__attribute__ ((noipa)) int
baz (unsigned short e)
{
  e <<= 1;
  e >>= __builtin_add_overflow (8719476735, u, &v);
  return e;
}

__attribute__ ((noipa)) int
qux (unsigned int e)
{
  c = ~1;
  c *= e;
  c = c >> (-15 & 5);
  return c + w + x;
}


int
main ()
{


  if (foo (0xffff) != 0x7fff)
    __builtin_abort ();
  if (bar (5) != 5)
    __builtin_abort ();
  if (baz (~0) != 0x7fff)
    __builtin_abort ();
  if (qux (2) != 0x7ffe)
    __builtin_abort ();

  return 0;
}
