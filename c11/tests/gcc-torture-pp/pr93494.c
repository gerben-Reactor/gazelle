# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93494.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93494.c" 2


unsigned short a;

int
main ()
{
  register unsigned long long y = 0;
  int x = __builtin_add_overflow (y, 0ULL, &a);
  if (x || a)
    __builtin_abort ();
  return 0;
}
