# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010604-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010604-1.c" 2
# 1 "/usr/lib/llvm-18/lib/clang/18/include/stdbool.h" 1 3
# 2 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20010604-1.c" 2

void abort (void);
void exit (int);

int f (int a, int b, int c, _Bool d, _Bool e, _Bool f, char g)
{
  if (g != 1 || d != 1 || e != 1 || f != 1) abort ();
  return a + b + c;
}

int main (void)
{
  if (f (1, 2, -3, 1, 1, 1, '\001'))
    abort ();
  exit (0);
}
