# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950714-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950714-1.c" 2
void abort (void);
void exit (int);

int array[10] = {1, 1, 1, 1, 1, 1, 1, 1, 1, 1};

int
main (void)
{
  int i, j;
  int *p;

  for (i = 0; i < 10; i++)
    for (p = &array[0]; p != &array[9]; p++)
      if (*p == i)
 goto label;

 label:
  if (i != 1)
    abort ();
  exit (0);
}
