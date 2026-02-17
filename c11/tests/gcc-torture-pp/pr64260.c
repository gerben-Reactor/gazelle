# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr64260.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr64260.c" 2


int a = 1, b;

void
foo (char p)
{
  int t = 0;
  for (; b < 1; b++)
    {
      int *s = &a;
      if (--t)
 *s &= p;
      *s &= 1;
    }
}

int
main ()
{
  foo (0);
  if (a != 0)
    __builtin_abort ();
  return 0;
}
