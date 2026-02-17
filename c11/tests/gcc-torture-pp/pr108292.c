# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr108292.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr108292.c" 2


typedef unsigned V __attribute__((__vector_size__ (64)));

V x;

int
main ()
{
  if (sizeof (unsigned) * 8 != 32)
    return 0;
  __builtin_sub_overflow (0, 6, &x[5]);
  x >>= ((V){} != x) & 31;
  for (unsigned i = 0; i < 16; i++)
    if (x[i] != (i == 5))
      __builtin_abort ();
  return 0;
}
