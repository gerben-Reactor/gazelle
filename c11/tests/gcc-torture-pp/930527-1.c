# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930527-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930527-1.c" 2


f (unsigned char x)
{
  return (0x50 | (x >> 4)) ^ 0xff;
}

main ()
{
  if (f (0) != 0xaf)
    abort ();
  exit (0);
}
