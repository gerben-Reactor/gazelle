# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr102134.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr102134.c" 2


typedef unsigned long long u64;

u64 g;

void
foo (u64 a, u64 b, u64 c, u64 *r)
{
  b *= b;
  u64 x = a && ((b >> (c & 63)) | ((b << (c & 63)) & g));
  *r = x + a;
}

int
main ()
{
  u64 x;
  foo (1, 3000, 0, &x);
  if (x != 2)
    __builtin_abort ();
  return 0;
}
