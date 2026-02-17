# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr89826.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr89826.c" 2
typedef unsigned int u32;
typedef unsigned long long u64;
u64 a;
u32 b;

u64
foo (u32 d)
{
  a -= d ? 0 : ~a;
  return a + b;
}

int
main (void)
{
  u64 x = foo (2);
  if (x != 0)
    __builtin_abort();
  return 0;
}
