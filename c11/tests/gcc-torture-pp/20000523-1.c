# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000523-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000523-1.c" 2
void abort (void);
void exit (int);

int
main (void)
{
  long long x;
  int n;

  if (sizeof (long long) < 8)
    exit (0);

  n = 9;
  x = (((long long) n) << 55) / 0xff;

  if (x == 0)
    abort ();

  x = (((long long) 9) << 55) / 0xff;

  if (x == 0)
    abort ();

  exit (0);
}
