# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr106523.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr106523.c" 2


__attribute__((noipa)) unsigned char
f7 (unsigned char x, unsigned int y)
{
  unsigned int t = x;
  return (t << y) | (t >> ((-y) & 7));
}

int
main ()
{
  if (8 != 8 || 4 != 4)
    return 0;

  volatile unsigned char x = 152;
  volatile unsigned int y = 19;
  if (f7 (x, y) != 4)
    __builtin_abort ();

  return 0;
}
