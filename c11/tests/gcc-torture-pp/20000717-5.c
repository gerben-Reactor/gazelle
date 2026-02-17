# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000717-5.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000717-5.c" 2
void abort (void);
void exit (int);

typedef struct trio { int a, b, c; } trio;

int
bar (int i, int j, int k, trio t)
{
  if (t.a != 1 || t.b != 2 || t.c != 3 ||
      i != 4 || j != 5 || k != 6)
    abort ();
}

int
foo (trio t, int i, int j, int k)
{
  return bar (i, j, k, t);
}

int
main (void)
{
  trio t = { 1, 2, 3 };

  foo (t, 4, 5, 6);
  exit (0);
}
