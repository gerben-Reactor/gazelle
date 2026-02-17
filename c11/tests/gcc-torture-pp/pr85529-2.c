# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr85529-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr85529-2.c" 2


__attribute__((noipa)) int
foo (int x)
{
  x &= 63;
  x -= 50;
  x |= 1;
  if (x < 0)
    return 1;
  int y = x >> 2;
  if (x >= y)
    return 1;
  return 0;
}

int
main ()
{
  int i;
  for (i = 0; i < 63; i++)
    if (foo (i) != 1)
      __builtin_abort ();
  return 0;
}
