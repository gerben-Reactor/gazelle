# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr21964-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr21964-1.c" 2
void abort (void);
void exit (int);

void
foo (int n, int m)
{
  if (m == 0)
    exit (0);
  else if (n != 0)
    abort ();
  else
    foo (n++, m - 1);
}

int
main (void)
{
  foo (0, 4);
}
