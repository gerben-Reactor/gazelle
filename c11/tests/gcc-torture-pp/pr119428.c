# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr119428.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr119428.c" 2


__attribute__((noipa)) void
foo (unsigned int x, unsigned char *y)
{
  y += x >> 3;
  *y &= (unsigned char) ~(1 << (x & 0x07));
}

int
main ()
{
  unsigned char buf[8];
  __builtin_memset (buf, 0xff, 8);
  foo (8, buf);
  if (buf[1] != 0xfe)
    __builtin_abort ();
}
