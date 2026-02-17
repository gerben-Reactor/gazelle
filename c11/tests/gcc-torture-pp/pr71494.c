# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr71494.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr71494.c" 2



int
main ()
{
  void *label = &&out;
  int i = 0;
  void test (void)
  {
    label = &&out2;
    goto *label;
   out2:;
    i++;
  }
  goto *label;
 out:
  i += 2;
  test ();
  if (i != 3)
    __builtin_abort ();
  return 0;
}
