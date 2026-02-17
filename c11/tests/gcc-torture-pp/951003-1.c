# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/951003-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/951003-1.c" 2
void abort (void);
void exit (int);

int f (int i) { return 12; }
int g () { return 0; }

int
main (void)
{
  int i, s;

  for (i = 0; i < 32; i++)
    {
      s = f (i);

      if (i == g ())
 s = 42;
      if (i == 0 || s == 12)
 ;
      else
 abort ();
    }

  exit (0);
}
