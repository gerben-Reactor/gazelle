# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98681.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98681.c" 2


__attribute__((noipa)) int
foo (int x)
{
  if (x > 32)
    return (x << -64) & 255;
  else
    return x;
}

int
main ()
{
  if (foo (32) != 32 || foo (-150) != -150)
    __builtin_abort ();
  return 0;
}
