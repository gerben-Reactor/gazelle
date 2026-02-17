# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97073.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr97073.c" 2



typedef unsigned long long L;
union U { L i; struct T { unsigned k; L l; } j; } u;

__attribute__((noinline,noclone)) void
foo (L x)
{
  u.j.l = u.i & x;
}

int
main ()
{
  u.i = 5;
  foo (-1ULL);
  if (u.j.l != 5)
    __builtin_abort ();
  return 0;
}
