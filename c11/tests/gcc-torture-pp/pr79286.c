# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr79286.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr79286.c" 2
int a = 0, c = 0;
static int d[][8] = {};

int main ()
{
  int e;
  for (int b = 0; b < 4; b++)
    {
      __builtin_printf ("%d\n", b, e);
      while (a && c++)
 e = d[300000000000000000][0];
    }

  return 0;
}
