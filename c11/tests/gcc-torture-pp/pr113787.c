# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr113787.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr113787.c" 2
void foo(int x, int y, int z, int d, int *buf)
{
  for(int i = z; i < y-z; ++i)
    for(int j = 0; j < d; ++j)

      buf[i*x+(x-z+j)] = buf[i*x+(x-z-1-j)];
}

void bar(int x, int y, int z, int d, int *buf)
{
  for(int i = 0; i < d; ++i)
    for(int j = z; j < x-z; ++j)

      buf[j+(y-z+i)*x] = buf[j+(y-z-1-i)*x];
}

__attribute__((noipa))
void baz(int x, int y, int d, int *buf)
{
  foo(x, y, 0, d, buf);
  bar(x, y, 0, d, buf);
}

int main(void)
{
  int a[] = { 1, 2, 3 };
  baz (1, 2, 1, a);







  for (int i = 0; i < 3; i++)
    if (a[i] != 1)
      __builtin_abort ();
}
