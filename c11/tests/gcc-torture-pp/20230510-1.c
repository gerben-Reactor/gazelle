# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20230510-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20230510-1.c" 2








_Bool __attribute__((noipa)) f(_Bool *a, _Bool b)
{
  _Bool t = *a;
  if (t <= b)
    return b;
  return t;
}
_Bool __attribute__((noipa)) f1(_Bool *a, _Bool b)
{
  return *a | b;
}

int main()
{
  int i = 0;
  int j = 0;

  for (i = 0; i <= 1; i++)
    for (j = 0; j <= 1; j++)
      {
        _Bool a = i;
        if (f(&a, j) != f1(&a, j))
          __builtin_abort();
      }
  return 0;
}
