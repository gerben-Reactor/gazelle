# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97386-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97386-1.c" 2


__attribute__((noipa)) unsigned char
foo (unsigned int c)
{
  return __builtin_bswap16 ((unsigned long long) (0xccccLLU << c | 0xccccLLU >> ((-c) & 63)));
}

int
main ()
{
  unsigned char x = foo (0);
  if (8 == 8 && 2 == 2 && x != 0xcc)
    __builtin_abort ();
  return 0;
}
