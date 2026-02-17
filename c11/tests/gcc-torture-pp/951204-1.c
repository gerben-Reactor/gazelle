# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/951204-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/951204-1.c" 2
void abort (void);
void exit (int);

void
f (char *x)
{
  *x = 'x';
}

int
main (void)
{
  int i;
  char x = '\0';

  for (i = 0; i < 100; ++i)
    {
      f (&x);
      if (*(const char *) &x != 'x')
 abort ();
    }
  exit (0);
}
