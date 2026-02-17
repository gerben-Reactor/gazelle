# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr94134.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr94134.c" 2


static volatile int a = 0;
static volatile int b = 1;

int
main ()
{
  a++;
  b++;
  if (a != 1 || b != 2)
    __builtin_abort ();
  return 0;
}
