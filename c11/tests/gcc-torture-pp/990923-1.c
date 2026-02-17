# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990923-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990923-1.c" 2
void abort (void);
void exit (int);




long
foo (long x)
{
  if ((x & 0xffff0000L) == 0xabcd0000L)
    return x & 0xffffL;
  return 1;
}

int
main (void)
{
  if (foo (0xabcd0000L) != 0 || foo (0) != 1)
    abort ();

  exit (0);
}
