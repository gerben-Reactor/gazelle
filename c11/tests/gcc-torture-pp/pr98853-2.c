# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98853-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98853-2.c" 2



__attribute__((noipa)) unsigned long long
foo (unsigned long long x, unsigned int y)
{
  return ((unsigned) x & 0xfffe0000U) | (y & 0x1ffff);
}


int
main ()
{

  if (foo (0xdeadbeefcaf2babeULL, 0xdeaffeedU) != 0x00000000caf3feedULL)
    __builtin_abort ();

  return 0;
}
