# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr40404.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr40404.c" 2
extern void abort (void);






struct S {
  unsigned int ui17 : 17;
} s;

int main()
{
  s.ui17 = 0x1ffff;
  if (s.ui17 >= 0xfffffffeu)
    abort ();
  return 0;
}
