# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/alias-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/alias-1.c" 2
int val;

int *ptr = &val;
float *ptr2 = (float *) &val;

__attribute__((optimize ("-fno-strict-aliasing")))
void
typepun (void)
{
  *ptr2=0;
}

int
main(void)
{
  *ptr=1;
  typepun ();
  if (*ptr)
    __builtin_abort ();
}
