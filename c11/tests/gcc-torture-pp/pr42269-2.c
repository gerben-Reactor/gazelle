# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr42269-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr42269-2.c" 2


static long long __attribute__((noinline))
foo (unsigned short s)
{
  return (short) s;
}

unsigned short s = 0xFFFF;

int
main (void)
{
  return foo (s) + 1 != 0;
}
