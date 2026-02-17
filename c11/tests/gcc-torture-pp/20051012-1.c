# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20051012-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20051012-1.c" 2


extern void abort (void);

struct type
{
  int *a;

  int b:16;
  unsigned int p:9;
} t;

unsigned int
foo ()
{
  return t.p;
}

int
main (void)
{
  t.p = 8;
  if (foo (t) != 8)
    abort ();
  return 0;
}
