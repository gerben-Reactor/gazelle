# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/enum-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/enum-2.c" 2



void abort (void);
void exit (int);

enum foo { FOO, BAR };







int
main ()
{
  int i;
  for (i = BAR; i >= FOO; --i)
    if (i == -1)
      abort ();

  exit (0);
}
