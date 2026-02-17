# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/scope-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/scope-1.c" 2
void abort (void);
void exit (int);

int v = 3;

void
f (void)
{
  int v = 4;
  {
    extern int v;
    if (v != 3)
      abort ();
  }
}

int
main (void)
{
  f ();
  exit (0);
}
