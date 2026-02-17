# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20100209-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20100209-1.c" 2
int bar(int foo)
{
  return (int)(((unsigned long long)(long long)foo) / 8);
}
extern void abort (void);
int main()
{
  if (sizeof (long long) > sizeof (int)
      && bar(-1) != -1)
    abort ();
  return 0;
}
