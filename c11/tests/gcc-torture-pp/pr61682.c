# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr61682.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr61682.c" 2


int a, b;
static int *c = &b;

int
main ()
{
  int *d = &a;
  for (a = 0; a < 12; a++)
    *c |= *d / 9;

  if (b != 1)
    __builtin_abort ();

  return 0;
}
