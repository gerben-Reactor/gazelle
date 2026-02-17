# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/complex-5.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/complex-5.c" 2
void abort (void);
void exit (int);

float __complex__
p (float __complex__ a, float __complex__ b)
{
  return a + b;
}

float __complex__ x = 1.0 + 14.0 * (1.0fi);
float __complex__ y = 7.0 + 5.0 * (1.0fi);
float __complex__ w = 8.0 + 19.0 * (1.0fi);
float __complex__ z;

int
main (void)
{

  z = p (x,y);
  y = p (x, 1.0f / z);
  if (z != w)
    abort ();
  exit (0);
}
