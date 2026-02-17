# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20051215-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20051215-1.c" 2


extern void abort (void);

__attribute__ ((noinline)) int
foo (int x, int y, int *z)
{
  int a, b, c, d;

  a = b = 0;
  for (d = 0; d < y; d++)
    {
      if (z)
 b = d * *z;
      for (c = 0; c < x; c++)
 a += b;
    }

  return a;
}

int
main (void)
{
  if (foo (3, 2, 0) != 0)
    abort ();
  return 0;
}
