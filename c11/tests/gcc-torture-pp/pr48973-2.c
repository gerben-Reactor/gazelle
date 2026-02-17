# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr48973-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr48973-2.c" 2


extern void abort (void);
struct S { int f : 1; } s;
int v = -1;

int
main ()
{
  s.f = v < 0;
  if ((unsigned int) s.f != -1U)
    abort ();
  return 0;
}
