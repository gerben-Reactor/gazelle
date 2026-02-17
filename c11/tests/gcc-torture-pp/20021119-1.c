# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20021119-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20021119-1.c" 2


extern void abort(void);

int foo (int i)
{
  int r;
  r = (80 - 4 * i) / 20;
  return r;
}

int main ()
{
  if (foo (1) != 3)
    abort ();
  return 0;
}
