# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20081103-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20081103-1.c" 2
struct S { char c; char arr[4]; float f; };

char A[4] = { '1', '2', '3', '4' };

void foo (struct S s)
{
  if (__builtin_memcmp (s.arr, A, 4))
    __builtin_abort ();
}

int main (void)
{
  struct S s;
  __builtin_memcpy (s.arr, A, 4);
  foo (s);
  return 0;
}
