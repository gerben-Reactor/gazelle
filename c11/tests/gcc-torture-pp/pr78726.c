# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr78726.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr78726.c" 2


unsigned char b = 36, c = 173;
unsigned int d;

__attribute__((noinline, noclone)) void
foo (void)
{
  unsigned a = ~b;
  d = a * c * c + 1023094746U * a;
}

int
main ()
{
  if (4 != 4 || 8 != 8)
    return 0;
  asm volatile ("" : : "g" (&b), "g" (&c) : "memory");
  foo ();
  if (d != 799092689U)
    __builtin_abort ();
  return 0;
}
