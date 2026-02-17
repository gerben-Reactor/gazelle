# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960302-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960302-1.c" 2
void abort (void);
void exit (int);

long a = 1;

int
foo (void)
{
  switch (a % 2 % 2 % 2 % 2 % 2 % 2 % 2 % 2)
    {
    case 0:
      return 0;
    case 1:
      return 1;
    default:
      return -1;
    }
}

int
main (void)
{
  if (foo () != 1)
    abort ();
  exit (0);
}
