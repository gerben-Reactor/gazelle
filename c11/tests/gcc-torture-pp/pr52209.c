# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr52209.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr52209.c" 2


extern void abort (void);
struct S0 { int f2 : 1; } c;
int b;

int
main ()
{
  b = -1 ^ c.f2;
  if (b != -1)
    abort ();
  return 0;
}
