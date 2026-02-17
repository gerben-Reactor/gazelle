# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr66187.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr66187.c" 2


int a = 1, e = -1;
short b, f;

int
main ()
{
  f = e;
  int g = b < 0 ? 0 : f + b;
  if ((g & -4) < 0)
    a = 0;
  if (a)
    __builtin_abort ();
  return 0;
}
