# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr119071.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr119071.c" 2


int a, b;

int
main ()
{
  int c = 0;
  if (a + 2)
    c = 1;
  int d = (1 + c - 2 + c == 1) - 1;
  b = ((d + 1) << d) + d;
  if (b != 1)
    __builtin_abort ();
}
