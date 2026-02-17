# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20050124-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20050124-1.c" 2


extern void abort (void);

int
foo (int i, int j)
{
  int k = i + 1;

  if (j)
    {
      if (k > 0)
 k++;
      else if (k < 0)
 k--;
    }

  return k;
}

int
main (void)
{
  if (foo (-2, 0) != -1)
    abort ();
  if (foo (-1, 0) != 0)
    abort ();
  if (foo (0, 0) != 1)
    abort ();
  if (foo (1, 0) != 2)
    abort ();
  if (foo (-2, 1) != -2)
    abort ();
  if (foo (-1, 1) != 0)
    abort ();
  if (foo (0, 1) != 2)
    abort ();
  if (foo (1, 1) != 3)
    abort ();
  return 0;
}
