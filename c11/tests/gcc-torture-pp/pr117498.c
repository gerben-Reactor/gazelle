# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr117498.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr117498.c" 2


int a, d, f;
char g;
volatile int c = 1;

int
foo ()
{
  if (c == 0)
    return -1;
  return 1;
}

void
bar (int h, int i, char *k, char *m)
{
  for (; d < i; d += 2)
    for (int j = 0; j < h; j++)
      m[j] = k[4 * j];
}

void
baz (long h)
{
  char n = 0;
  bar (h, 4, &n, &g);
}

int
main ()
{
  f = foo ();
  baz ((unsigned char) f - 4);
}
