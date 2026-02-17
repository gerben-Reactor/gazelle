# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950906-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/950906-1.c" 2
void abort (void);
void exit (int);

void
g (int i)
{
}

void
f (int i)
{
  g (0);
  while ( ({ i--; }) )
    g (0);
}

int
main (void)
{
  f (10);
  exit (0);
}
