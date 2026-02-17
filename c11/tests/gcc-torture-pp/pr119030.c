# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr119030.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr119030.c" 2


static inline unsigned
foo (long long x)
{
  return x & 0x8000;
}

static inline long long
bar (long long x)
{
  if (foo (x))
    return -1000L;
  else
    return x >> 16;
}

long long x = -0x20000LL;

int
main ()
{
  if (bar (x) >= 0)
    __builtin_abort ();
  return 0;
}
