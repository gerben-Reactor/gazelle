# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr78436.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr78436.c" 2


struct S
{
  long int a : 24;
  signed char b : 8;
} s;

__attribute__((noinline, noclone)) void
foo ()
{
  s.b = 0;
  s.a = -1193165L;
}

int
main ()
{
  foo ();
  if (s.b != 0)
    __builtin_abort ();
  return 0;
}
