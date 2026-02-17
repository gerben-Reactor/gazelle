# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960512-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960512-1.c" 2
void abort (void);
void exit (int);

__complex__
double f ()
{
  int a[40];
  __complex__ double c;

  a[9] = 0;
  c = a[9];
  return c;
}

int
main (void)
{
  __complex__ double c;

  if (c = f ())
    abort ();
  exit (0);
}
