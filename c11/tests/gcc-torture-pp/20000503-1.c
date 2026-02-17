# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000503-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000503-1.c" 2
void abort (void);
void exit (int);

unsigned long
sub (int a)
{
  return ((0 > a - 2) ? 0 : a - 2) * sizeof (long);
}

int
main (void)
{
  if (sub (0) != 0)
    abort ();

  exit (0);
}
