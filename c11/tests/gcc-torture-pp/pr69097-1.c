# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr69097-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr69097-1.c" 2


int a, b;
unsigned int c;

int
main ()
{
  int d = b;
  b = ~(~a + (~d | b));
  a = ~(~c >> b);
  c = a % b;
  return 0;
}
