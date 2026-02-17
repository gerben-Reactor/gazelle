# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20060110-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20060110-2.c" 2
extern void abort (void);

long long
f (long long a, long long b)
{
  return ((a + b) << 32) >> 32;
}

long long a = 0x1234567876543210LL;
long long b = 0x2345678765432101LL;
long long c = ((0x1234567876543210LL + 0x2345678765432101LL) << 32) >> 32;

int
main ()
{
  if (f (a, b) != c)
    abort ();
  return 0;
}
