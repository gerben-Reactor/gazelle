# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr79043.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr79043.c" 2


int val;

int *ptr = &val;
float *ptr2 = (float *) &val;

static void
__attribute__((always_inline, optimize ("-fno-strict-aliasing")))
typepun ()
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
