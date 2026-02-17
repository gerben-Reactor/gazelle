# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920612-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920612-2.c" 2


void abort (void);
void exit (int);

int
main (void)
{
  int i = 0;
  int a (int x)
    {
      while (x)
 i++, x--;
      return x;
    }

  if (a (2) != 0)
    abort ();

  exit (0);
}
