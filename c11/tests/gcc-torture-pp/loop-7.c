# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/loop-7.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/loop-7.c" 2
void abort (void);
void exit (int);

void foo (unsigned int n)
{
  int i, j = -1;

  for (i = 0; i < 10 && j < 0; i++)
    {
      if ((1UL << i) == n)
 j = i;
    }

  if (j < 0)
    abort ();
}

int
main(void)
{
  foo (64);
  exit (0);
}
