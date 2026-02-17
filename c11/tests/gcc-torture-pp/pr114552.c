# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr114552.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr114552.c" 2


struct __attribute__((packed)) S { short b; int c; };
struct T { struct S b; int e; };
static const struct T k = { { 1, 0 }, 0 };

__attribute__((noinline)) void
foo (void)
{
  asm volatile ("" : : : "memory");
}

__attribute__((noinline)) void
bar (struct S n)
{
  foo ();
}

int
main ()
{
  bar (k.b);
  return 0;
}
