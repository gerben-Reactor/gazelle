# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20001121-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20001121-1.c" 2


extern void abort (void);
extern void exit (int);

double d;

__inline__ double foo (void)
{
  return d;
}

__inline__ int bar (void)
{
  foo();
  return 0;
}

int main (void)
{
  if (bar ())
    abort ();
  exit (0);
}
