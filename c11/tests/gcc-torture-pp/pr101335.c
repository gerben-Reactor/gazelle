# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr101335.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr101335.c" 2

unsigned a = 0xFFFFFFFF;
int b;
int main()
{
  int c = ~a;
  unsigned d = c - 10;
  if (d > c)
    c = 20;
  b = -(c | 0);
  if (b > -8)
    __builtin_abort ();
  return 0;
}
