# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030828-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030828-1.c" 2
void abort (void);
void exit (int);

const int *p;

int bar (void)
{
  return *p + 1;
}

int
main (void)
{



  const int i = 5;
  p = &i;
  if (bar() != 6)
    abort ();
  exit (0);
}
