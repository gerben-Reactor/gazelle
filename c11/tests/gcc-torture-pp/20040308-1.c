# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040308-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20040308-1.c" 2


void foo(int n)
{
  struct S {
    int i[n];
    unsigned int b:1;
    int i2;
  } __attribute__ ((packed)) __attribute__ ((aligned (4)));

  struct S s;

  s.i2 = 0;
}

int main(void)
{
  foo(4);

  return 0;
}
