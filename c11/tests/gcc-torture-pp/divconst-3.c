# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/divconst-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/divconst-3.c" 2
void abort (void);
void exit (int);

long long
f (long long x)
{
  return x / 10000000000LL;
}

int
main (void)
{
  if (f (10000000000LL) != 1 || f (100000000000LL) != 10)
    abort ();
  exit (0);
}
