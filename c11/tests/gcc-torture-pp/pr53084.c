# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr53084.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr53084.c" 2


extern void abort (void);

__attribute__((noinline, noclone)) void
bar (const char *p)
{
  if (p[0] != 'o' || p[1] != 'o' || p[2])
    abort ();
}

int
main ()
{
  static const char *const foo[] = {"foo" + 1};
  bar (foo[0]);
  return 0;
}
