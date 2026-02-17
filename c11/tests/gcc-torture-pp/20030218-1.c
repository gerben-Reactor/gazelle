# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030218-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030218-1.c" 2




extern void abort ();
extern void exit (int);

short *q;

long
foo (short *p)
{
  long b = *p;
  q = p + 1;
  return b;
}

int
main ()
{
  short a = 0xff00;
  if (foo (&a) != (long) (short) 0xff00)
    abort ();
  exit (0);
}
