# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr49186.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr49186.c" 2

extern void abort (void);

int
main ()
{
  int x;
  unsigned long long uv = 0x1000000001ULL;

  x = (uv < 0x80) ? 1 : ((uv < 0x800) ? 2 : 3);
  if (x != 3)
    abort ();

  return 0;
}
