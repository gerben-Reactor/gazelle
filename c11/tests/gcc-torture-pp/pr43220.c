# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr43220.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr43220.c" 2


void *volatile p;

int
main (void)
{
  int n = 0;
lab:;
    {
      int x[n % 1000 + 1];
      x[0] = 1;
      x[n % 1000] = 2;
      p = x;
      n++;
    }

    {
      int x[n % 1000 + 1];
      x[0] = 1;
      x[n % 1000] = 2;
      p = x;
      n++;
    }

  if (n < 1000000)
    goto lab;

  return 0;
}
