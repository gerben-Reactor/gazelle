# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr99751.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr99751.c" 2
int *ptr1 = 0, **ptr2 = &ptr1;

int *identity(int *p)
{
  return p;
}

void store_to_c(int *p)
{
  *ptr2 = identity(p);
}

int main()
{
  int f;
  store_to_c(&f);
  if (ptr1 != &f)
    __builtin_abort();
  return 0;
}
