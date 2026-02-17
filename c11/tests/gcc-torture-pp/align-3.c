# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/align-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/align-3.c" 2


void abort (void);

void func(void) __attribute__((aligned(256)));

void func(void)
{
}

int main()
{
  if (__alignof__(func) != 256)
    abort ();
  return 0;
}
