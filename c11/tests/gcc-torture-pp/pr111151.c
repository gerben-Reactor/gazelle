# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111151.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr111151.c" 2


int
main ()
{
  unsigned a = (1U + 2147483647) / 2U;
  unsigned b = 1U;
  unsigned c = (a * 2U > b * 2U ? a * 2U : b * 2U) * 2U;
  if (c != 0U)
    __builtin_abort ();
  int d = (-2147483647 - 1) / 2;
  int e = 10;
  int f = (d * 2 > e * 5 ? d * 2 : e * 5) * 6;
  if (f != 300)
    __builtin_abort ();
  int g = (-2147483647 - 1) / 2;
  int h = 0;
  int i = (g * 2 > h * 5 ? g * 2 : h * 5) / -1;
  if (i != 0)
    __builtin_abort ();
}
