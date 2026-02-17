# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930526-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930526-1.c" 2


extern void exit (int);

inline void
f (int x)
{
  int *(p[25]);
  int m[25*7];
  int i;

  for (i = 0; i < 25; i++)
    p[i] = m + x*i;

  p[1][0] = 0;
}

int
main ()
{
  f (7);
  exit (0);
}
