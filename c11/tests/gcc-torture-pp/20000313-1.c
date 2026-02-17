# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000313-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000313-1.c" 2
void abort (void);

unsigned int buggy (unsigned int *param)
{
  unsigned int accu, zero = 0, borrow;
  accu = - *param;
  borrow = - (accu > zero);
  *param += accu;
  return borrow;
}

int main (void)
{
  unsigned int param = 1;
  unsigned int borrow = buggy (&param);

  if (param != 0)
    abort ();
  if (borrow + 1 != 0)
    abort ();
  return 0;
}
