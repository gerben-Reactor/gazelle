# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr36077.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr36077.c" 2
extern void abort (void);

unsigned int test (unsigned int x)
{
  return x / 0x80000001U / 0x00000002U;
}

int main()
{
  if (test(2) != 0)
    abort ();
  return 0;
}
