# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr71554.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr71554.c" 2


int v;

__attribute__ ((noinline, noclone)) void
bar (void)
{
  v++;
}

__attribute__ ((noinline, noclone))
void
foo (unsigned int x)
{
  signed int y = ((-2147483647 - 1) / 2);
  signed int r;
  if (__builtin_mul_overflow (x, y, &r))
    bar ();
}

int
main ()
{
  foo (2);
  if (v)
    __builtin_abort ();
  return 0;
}
