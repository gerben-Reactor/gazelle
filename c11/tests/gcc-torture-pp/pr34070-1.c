# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr34070-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr34070-1.c" 2
extern void abort (void);

int f(unsigned int x)
{
    return ((int)x) % 4;
}

int main()
{
  if (f(-1) != -1)
    abort ();
  return 0;
}
