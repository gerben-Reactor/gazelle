# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960321-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960321-1.c" 2
void abort (void);
void exit (int);

char a[10] = "deadbeef";

char
acc_a (long i)
{
  return a[i-2000000000L];
}

int
main (void)
{
  if (acc_a (2000000000L) != 'd')
    abort ();
  exit (0);
}
