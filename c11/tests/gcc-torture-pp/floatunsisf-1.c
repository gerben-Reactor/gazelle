# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/floatunsisf-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/floatunsisf-1.c" 2




extern void abort (void);
extern void exit (int);

volatile unsigned u = 0x80000081;



volatile float f1, f2;
int
main (void)
{
  f1 = (float) u;
  f2 = (float) 0x80000081;
  if (f1 != f2)
    abort ();
  exit (0);
}
