# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/nestfunc-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/nestfunc-1.c" 2


void abort (void);
void exit (int);

int
g (int a, int b, int (*gi) (int, int))
{
  if ((*gi) (a, b))
    return a;
  else
    return b;
}

void
f ()
{
  int i, j;
  int f2 (int a, int b)
    {
      return a > b;
    }

  if (g (1, 2, f2) != 2)
    abort ();
}

int
main (void)
{
  f ();
  exit (0);
}
