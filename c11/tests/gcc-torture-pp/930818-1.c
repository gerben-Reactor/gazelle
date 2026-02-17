# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930818-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930818-1.c" 2


static double one = 1.0;

f()
{
  int colinear;
  colinear = (one == 0.0);
  if (colinear)
    abort ();
  return colinear;
}
main()
{
  if (f()) abort();
  exit (0);
}
