# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/941025-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/941025-1.c" 2
void abort (void);
void exit (int);

long f (x, y)
     long x,y;
{
  return (x > 1) ? y : (y & 1);
}

int
main (void)
{
  if (f (2L, 0xdecadeL) != 0xdecadeL)
    abort ();
  exit (0);
}
