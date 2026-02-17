# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/memset-4.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/memset-4.c" 2




void f(char *a) __attribute__((noinline));
void f(char *a)
{
  __builtin_memset (a, 0, 15);
}


int main(void)
{
  int i;
  char b[15];
  for(i = 0; i < sizeof(b); i++)
    {
      b[i] = i;
    }
  f(b);
  for(i = 0; i < sizeof(b); i++)
    {
      if (0 != b[i])
 __builtin_abort ();
    }
  return 0;
}
