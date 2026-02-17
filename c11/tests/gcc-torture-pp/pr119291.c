# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr119291.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr119291.c" 2


int a;
long c;

__attribute__((noipa)) void
foo (int x)
{
  if (x != 0)
    __builtin_abort ();
  a = 42;
}

int
main ()
{
  int e = 1;
lab:
  if (a < 2)
    {
      int b = e;
      _Bool d = a != 0;
      _Bool f = b != 0;
      unsigned long g = -(d & f);
      unsigned long h = c & g;
      unsigned long i = ~c;
      e = -(i & h);
      c = e != 0;
      a = ~e + b;
      foo (e);
      goto lab;
    }
}
