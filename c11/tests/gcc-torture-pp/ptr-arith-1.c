# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/ptr-arith-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/ptr-arith-1.c" 2
void abort (void);
void exit (int);

char *
f (char *s, unsigned int i)
{
  return &s[i + 3 - 1];
}

int
main (void)
{
  char *str = "abcdefghijkl";
  char *x2 = f (str, 12);
  if (str + 14 != x2)
    abort ();
  exit (0);
}
