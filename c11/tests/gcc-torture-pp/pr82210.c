# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr82210.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr82210.c" 2


void
foo (int size)
{
  int i;
  struct S {
    __attribute__((aligned (16))) struct T { short c; } a[size];
    int b[size];
  } s;

  for (i = 0; i < size; i++)
    s.a[i].c = 0x1234;
  for (i = 0; i < size; i++)
    s.b[i] = 0;
  for (i = 0; i < size; i++)
    if (s.a[i].c != 0x1234 || s.b[i] != 0)
      __builtin_abort ();
}

int
main ()
{
  foo (15);
  return 0;
}
