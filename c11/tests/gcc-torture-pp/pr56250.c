# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr56250.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr56250.c" 2


extern void abort (void);

int
main ()
{
  unsigned int x = 2;
  unsigned int y = (0U - x / 2) / 2;
  if (-1U / x != y)
    abort ();
  return 0;
}
