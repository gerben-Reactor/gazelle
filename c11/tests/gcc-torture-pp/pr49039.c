# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr49039.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr49039.c" 2

extern void abort (void);
int cnt;

__attribute__((noinline, noclone)) void
foo (unsigned int x, unsigned int y)
{
  unsigned int minv, maxv;
  if (x == 1 || y == -2U)
    return;
  minv = x < y ? x : y;
  maxv = x > y ? x : y;
  if (minv == 1)
    ++cnt;
  if (maxv == -2U)
    ++cnt;
}

int
main ()
{
  foo (-2U, 1);
  if (cnt != 2)
    abort ();
  return 0;
}
