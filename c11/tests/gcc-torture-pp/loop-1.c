# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/loop-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/loop-1.c" 2
void abort (void);
void exit (int);

int
main (void)
{
  int i, j, k[3];

  j = 0;
  for (i=0; i < 3; i++)
    {
      k[i] = j++;
    }

  for (i=2; i >= 0; i--)
    {
      if (k[i] != i)
 abort ();
    }

  exit (0);
}
