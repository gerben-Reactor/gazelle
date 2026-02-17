# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20090219-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20090219-1.c" 2



extern void abort (void);
extern void exit (int);

void
foo (void)
{
  int f = 0;

  void bar (int a, int b, int c, int d, int e)
    {
      if (e != 0)
 {
   f = 1;
   abort ();
 }
    }

  bar (0, 0, 0, 0, 0);
}

int
main (void)
{
  foo ();
  exit (0);
}
