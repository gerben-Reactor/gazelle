# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20041126-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20041126-1.c" 2
extern int abs (int);
extern void abort (void);

void
check (int *p)
{
  int i;
  for (i = 0; i < 5; ++i)
    if (p[i])
      abort ();
  for (; i < 10; ++i)
    if (p[i] != i + 1)
      abort ();
}

int
main (void)
{
  int a[10] = { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
  int i;

  for (i = -5; i < 0; i++)
    a[abs (i - 10) - 11] = 0;
  check (a);
  return 0;
}
