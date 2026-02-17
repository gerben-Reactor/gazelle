# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr80501.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr80501.c" 2


signed char v = 0;

static signed char
foo (int x, int y)
{
  return x << y;
}

__attribute__((noinline, noclone)) int
bar (void)
{
  return foo (v >= 0, 8 - 1) >= 1;
}

int
main ()
{
  if (sizeof (int) > sizeof (char) && bar () != 0)
    __builtin_abort ();
  return 0;
}
