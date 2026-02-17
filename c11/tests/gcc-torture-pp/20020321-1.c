# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020321-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020321-1.c" 2





float g (void *a, void *b, int e, int c, float d)
{
  return d;
}

float f (void *a, void *b, int c, float d)
{
  return g (a, b, 0, c, d);
}

int main ()
{
  f (0, 0, 1, 1);
  return 0;
}
