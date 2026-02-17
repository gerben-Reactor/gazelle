# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr65053-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr65053-1.c" 2


int i;

__attribute__ ((noinline, noclone))
unsigned int foo (void)
{
  return 0;
}

int
main ()
{
  unsigned int u = -1;
  if (u == -1)
    {
      unsigned int n = foo ();
      if (n > 0)
 u = n - 1;
    }

  while (u != -1)
    {
      asm ("" : "+g" (u));
      u = -1;
      i = 1;
    }

  if (i)
    __builtin_abort ();
  return 0;
}
