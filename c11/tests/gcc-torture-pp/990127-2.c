# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990127-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990127-2.c" 2


extern void abort (void);
extern void exit (int);

void
fpEq (double x, double y)
{
  if (x != y)
    abort ();
}

void
fpTest (double x, double y)
{
  double result1 = (35.7 * 100.0) / 45.0;
  double result2 = (x * 100.0) / y;
  fpEq (result1, result2);
}

int
main ()
{
  fpTest (35.7, 45.0);
  exit (0);
}
