# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr59221.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr59221.c" 2


int a = 1, b, d;
short e;

int
main ()
{
  for (; b; b++)
    ;
  short f = a;
  int g = 15;
  e = f ? f : 1 << g;
  int h = e;
  d = h == 83647 ? 0 : h;
  if (d != 1)
    __builtin_abort ();
  return 0;
}
