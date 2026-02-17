# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr67929_1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr67929_1.c" 2
int __attribute__ ((noinline, noclone))
foo (float a)
{
  return a * 4.9f;
}


int
main (void)
{
  if (foo (10.0f) != 49)
    __builtin_abort ();

  return 0;
}
