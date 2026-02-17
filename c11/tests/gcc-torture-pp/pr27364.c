# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr27364.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr27364.c" 2
void exit (int);

int f(unsigned number_of_digits_to_use)
{
  if (number_of_digits_to_use >1294)
    return 0;
  return (number_of_digits_to_use * 3321928 / 1000000 + 1) /16;
}

int main(void)
{
  if (f(11) != 2)
    __builtin_abort ();
  exit (0);
}
