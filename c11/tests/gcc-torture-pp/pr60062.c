# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr60062.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr60062.c" 2


int a;

static void
foo (const char *p1, int p2)
{
  if (__builtin_strcmp (p1, "hello") != 0)
    __builtin_abort ();
}

static void
bar (const char *p1)
{
  if (__builtin_strcmp (p1, "hello") != 0)
    __builtin_abort ();
}

__attribute__((optimize (0))) int
main ()
{
  foo ("hello", a);
  bar ("hello");
  return 0;
}
