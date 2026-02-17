# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57281.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57281.c" 2


int a = 1, b, d, *e = &d;
long long c, *g = &c;
volatile long long f;

int
foo (int h)
{
  int j = *g = b;
  return h == 0 ? j : 0;
}

int
main ()
{
  int h = a;
  for (; b != -20; b--)
    {
      (int) f;
      *e = 0;
      *e = foo (h);
    }
  return 0;
}
