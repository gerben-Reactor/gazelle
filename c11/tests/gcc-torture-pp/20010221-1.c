# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010221-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010221-1.c" 2
void abort (void);
void exit (int);

int n = 2;

int
main (void)
{
  int i, x = 45;

  for (i = 0; i < n; i++)
    {
      if (i != 0)
 x = ( i > 0 ) ? i : 0;
    }

  if (x != 1)
    abort ();
  exit (0);
}
