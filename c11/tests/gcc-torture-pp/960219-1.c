# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960219-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/960219-1.c" 2
void abort (void);
void exit (int);

void
f (int i)
{
  if (((1 << i) & 1) == 0)
    abort ();
}

int
main (void)
{
  f (0);
  exit (0);
}
