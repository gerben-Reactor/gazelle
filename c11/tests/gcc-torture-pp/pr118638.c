# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr118638.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr118638.c" 2


__attribute__((noipa)) int
foo (int x)
{
  int a = x != -3, b, c;
  a *= 3;
  b = 2 * x - 9;
  a = a + b;
  a = ~a;
  c = a & 1;
  return -c;
}

int
main ()
{
  if (foo (0) != -1)
    __builtin_abort ();
}
