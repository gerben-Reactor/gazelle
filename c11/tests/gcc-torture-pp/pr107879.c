# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr107879.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr107879.c" 2


__attribute__((noipa)) static double
foo (double *y)
{
  volatile int ph = 0;
  volatile double vf = 1.0;
  double factor = vf;
  double x = - (double) ph * factor;
  if (x == 0)
    *y = 1.0;
  else
    *y = 1.0 / x;
  double w = 2.0 * x / factor;
  double omww = 1 - w;
  return omww > 0.0 ? omww : 0.0;
}

int
main ()
{
  double y = 42.0;
  if (foo (&y) != 1.0)
    __builtin_abort ();
}
