# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr35231.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr35231.c" 2
extern void abort(void);

int __attribute__((noinline))
foo(int bits_per_pixel, int depth)
{
  if ((bits_per_pixel | depth) == 1)
    abort ();
  return bits_per_pixel;
}

int main()
{
  if (foo(2, 0) != 2)
    abort ();
  return 0;
}
