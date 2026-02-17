# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr94809.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr94809.c" 2


int
main ()
{
  int a = 0;
  unsigned long long one = 1;
  ((-1ULL / one) < a++, one);
  if (a != 1)
    __builtin_abort ();
  return 0;
}
