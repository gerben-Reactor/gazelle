# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr23941.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr23941.c" 2
extern void abort (void);
double d = 1.17549435e-38F / 2.0;
int main()
{
  double x = 1.17549435e-38F / 2.0;
  if (x != d)
    abort ();
  return 0;
}
