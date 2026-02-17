# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr34971.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr34971.c" 2
struct foo
{
  unsigned long long b:40;
} x;

extern void abort (void);

void test1(unsigned long long res)
{

  if ((x.b<<8) + (x.b>>32) != res)
    abort ();
}

int main()
{
  x.b = 0x0100000001;
  test1(0x0000000101);
  x.b = 0x0100000000;
  test1(0x0000000001);
  return 0;
}
