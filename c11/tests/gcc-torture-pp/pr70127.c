# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr70127.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr70127.c" 2


struct S { int f; signed int g : 2; } a[1], c = {5, 1}, d;
short b;

__attribute__((noinline, noclone)) void
foo (int x)
{
  if (x != 1)
    __builtin_abort ();
}

int
main ()
{
  while (b++ <= 0)
    {
      struct S e = {1, 1};
      d = e = a[0] = c;
    }
  foo (a[0].g);
  return 0;
}
