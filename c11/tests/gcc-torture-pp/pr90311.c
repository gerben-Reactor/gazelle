# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr90311.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr90311.c" 2


int a, b;

int
main ()
{
  unsigned long long x;
  unsigned int c;
  __builtin_add_overflow ((unsigned char) a, b, &c);
  b -= c < (unsigned char) a;
  x = b;
  if (x)
    __builtin_abort ();
  return 0;
}
