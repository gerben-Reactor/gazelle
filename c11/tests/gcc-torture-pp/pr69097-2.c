# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr69097-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr69097-2.c" 2


__attribute__((noinline, noclone)) int
f1 (int x, int y)
{
  return x % y;
}

__attribute__((noinline, noclone)) int
f2 (int x, int y)
{
  return x % -y;
}

__attribute__((noinline, noclone)) int
f3 (int x, int y)
{
  int z = -y;
  return x % z;
}

int
main ()
{
  if (f1 (-2147483647 - 1, 1) != 0
      || f2 (-2147483647 - 1, -1) != 0
      || f3 (-2147483647 - 1, -1) != 0)
    __builtin_abort ();
  return 0;
}
