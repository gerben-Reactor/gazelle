# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98366.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr98366.c" 2



typedef struct S { int a, b, c : 7, d : 8, e : 17; } S;
const S f[] = { {0, 3, 4, 2, 0} };

int
main ()
{
  if (__builtin_memcmp (f, (S[]){{.b = 3, .c = 4, .d = 2, .e = 0}}, sizeof (S)))
    __builtin_abort ();
  return 0;
}
