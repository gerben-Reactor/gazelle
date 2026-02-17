# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr31136.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr31136.c" 2
extern void abort (void);

struct S {
  unsigned b4:4;
  unsigned b6:6;
} s;

int main()
{
  s.b6 = 31;
  s.b4 = s.b6;
  s.b6 = s.b4;
  if (s.b6 != 15)
    abort ();
  return 0;
}
