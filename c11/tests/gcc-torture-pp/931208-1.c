# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931208-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/931208-1.c" 2

f ()
{
  unsigned long x, y = 1;

  x = ((y * 8192) - 216) / 16;
  return x;
}

main ()
{
  if (f () != 498)
    abort ();
  exit (0);
}
