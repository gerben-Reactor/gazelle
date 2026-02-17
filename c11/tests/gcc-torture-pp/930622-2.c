# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930622-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930622-2.c" 2
void abort (void);
void exit (int);

long double
ll_to_ld (long long n)
{
  return n;
}

long long
ld_to_ll (long double n)
{
  return n;
}

int
main (void)
{
  long long n;

  if (ll_to_ld (10LL) != 10.0)
    abort ();

  if (ld_to_ll (10.0) != 10)
    abort ();

  exit (0);
}
