# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97386-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97386-2.c" 2


__attribute__((noipa)) unsigned
foo (int x)
{
  unsigned long long a = (0x800000000000ccccULL << x) | (0x800000000000ccccULL >> (64 - x));
  unsigned int b = a;
  return (b << 24) | (b >> 8);
}

int
main ()
{
  if (8 == 8
      && 4 == 4
      && 8 == 8
      && foo (1) != 0x99000199U)
    __builtin_abort ();
  return 0;
}
