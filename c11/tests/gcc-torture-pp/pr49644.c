# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr49644.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr49644.c" 2


extern void abort (void);

int
main ()
{
  _Complex double a[12], *c = a, s = 3.0 + 1.0i;
  double b[12] = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12 }, *d = b;
  int i;
  for (i = 0; i < 6; i++)
    *c++ = *d++ * s;
  if (c != a + 6 || d != b + 6)
    abort ();
  return 0;
}
