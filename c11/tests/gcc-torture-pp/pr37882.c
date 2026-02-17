# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr37882.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr37882.c" 2


struct S
{
  unsigned char b : 3;
} s;

int
main ()
{
  s.b = 4;
  if (s.b > 0 && s.b < 4)
    __builtin_abort ();
  return 0;
}
