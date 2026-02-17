# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040331-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040331-1.c" 2

extern void abort (void);
extern void exit (int);

int
main (void)
{

  struct { int count: 31; } s = { 0 };
  while (s.count--)
    abort ();







  exit (0);
}
