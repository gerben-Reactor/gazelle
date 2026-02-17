# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20080506-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20080506-1.c" 2

extern void abort (void);




int
main ()
{
  unsigned int u;
  int i = -1;

  u = (((unsigned int) ((i) > (0) ? (i) : (0))) > (1) ? ((unsigned int) ((i) > (0) ? (i) : (0))) : (1));
  if (u != 1)
    abort ();

  u = (((unsigned int) ((i) > (0) ? (i) : (0))) < ((unsigned int) i) ? ((unsigned int) ((i) > (0) ? (i) : (0))) : ((unsigned int) i));
  if (u != 0)
    abort ();
  return 0;
}
