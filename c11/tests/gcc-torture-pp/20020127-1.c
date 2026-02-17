# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020127-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020127-1.c" 2


extern void abort (void);
extern void exit (int);

unsigned long
foo (unsigned long n)
{
  return (~n >> 3) & 1;
}

int
main ()
{
  if (foo (1 << 3) != 0)
    abort ();

  if (foo (0) != 1)
    abort ();

  exit (0);
}
