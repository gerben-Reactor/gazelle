# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr42721.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr42721.c" 2


extern void abort (void);

static unsigned long long
foo (unsigned long long x, unsigned long long y)
{
  return x / y;
}

static int a, b;

int
main (void)
{
  unsigned long long c = 1;
  b ^= c && (foo (a, -1ULL) != 1L);
  if (b != 1)
    abort ();
  return 0;
}
