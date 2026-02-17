# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20031214-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20031214-1.c" 2






struct
{
  double a;
  int n[2];
} g = { 0., { 1, 2}};

int k = 0;

void
b (int *j)
{
}

int
main ()
{
  int j;

  for (j = 0; j < 2; j++)
    k = (k > g.n[j]) ? k : g.n[j];

  k++;
  b (&j);

  return 0;
}
