# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97421-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97421-2.c" 2



static int a, b, c;
int *d = &c;
int **e = &d;
int ***f = &e;
int main()
{
  int h;
  for (a = 2; a; a--)
    for (h = 0; h <= 2; h++)
      for (b = 0; b <= 2; b++)
        ***f = 6;

  if (b != 3)
    __builtin_abort();
}
