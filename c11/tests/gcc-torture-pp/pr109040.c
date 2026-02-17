# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr109040.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr109040.c" 2


typedef unsigned short __attribute__((__vector_size__ (32))) V;

unsigned short a, b, c, d;

void
foo (V m, unsigned short *ret)
{
  V v = 6 > ((V) { 2124, 8 } & m);
  unsigned short uc = v[0] + a + b + c + d;
  *ret = uc;
}

int
main ()
{
  unsigned short x;
  foo ((V) { 0, 15 }, &x);
  if (x != (unsigned short) ~0)
    __builtin_abort ();
  return 0;
}
