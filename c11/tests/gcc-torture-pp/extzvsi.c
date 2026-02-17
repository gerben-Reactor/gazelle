# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/extzvsi.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/extzvsi.c" 2


void abort (void);
void exit (int);
# 15 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/extzvsi.c"
struct ieee
{
  unsigned int negative:1;
  unsigned int exponent:11;
  unsigned int mantissa0:20;
  unsigned int mantissa1:32;
} x;

unsigned int
foo (void)
{
  unsigned int exponent;

  exponent = x.exponent;
  if (exponent == 0)
    return 1;
  else if (exponent > 1)
    return 2;
  return 0;
}

int
main (void)
{
  x.exponent = 1;
  if (foo () != 0)
    abort ();
  return 0;
}
