# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20190228-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20190228-1.c" 2



int a = 1;

int main (void)
{
  a = ~(a && 1);
  if (a < -1)
    a = ~a;

  if (!a)
    __builtin_abort ();

  return 0;
}
