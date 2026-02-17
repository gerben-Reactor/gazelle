# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57344-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr57344-2.c" 2



struct __attribute__((packed)) S
{
  int a : 27;

  int b : 22;



  char c;
  int : 0;
} s[2];
int i;

__attribute__((noinline, noclone)) void
foo (int x)
{
  if (x != -3161)
    __builtin_abort ();
  asm volatile ("" : : : "memory");
}

int
main ()
{
  struct S t = { 0, -3161L };
  s[1] = t;
  for (; i < 1; i++)
    foo (s[1].b);
  return 0;
}
