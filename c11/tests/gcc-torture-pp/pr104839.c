# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr104839.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr104839.c" 2


__attribute__((noipa)) short
foo (void)
{
  return -1;
}

__attribute__((noipa)) int
bar (void)
{
  short i = foo ();
  if (i == -2)
    return 2;
  long k = i;
  int j = -1;
  volatile long s = 300;
  if (k < 0)
    {
      k += s;
      if (k < 0)
 j = 0;
    }
  else if (k >= s)
    j = 0;
  if (j != -1)
    return 1;
  return 0;
}

int
main ()
{
  if (bar () != 0)
    __builtin_abort ();
  return 0;
}
