# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93582.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93582.c" 2


short a;
int b, c;

__attribute__((noipa)) void
foo (void)
{
  b = c;
  a &= 7;
}

int
main ()
{
  c = 27;
  a = 14;
  foo ();
  if (b != 27 || a != 6)
    __builtin_abort ();
  return 0;
}
