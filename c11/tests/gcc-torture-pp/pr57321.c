# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57321.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57321.c" 2


int a = 1, *b, **c;

static int
foo (int *p)
{
  if (*p == a)
    {
      int *i[7][5] = { { 0 } };
      int **j[1][1];
      j[0][0] = &i[0][0];
      *b = &p != c;
    }
  return 0;
}

int
main ()
{
  int i = 0;
  foo (&i);
  return 0;
}
