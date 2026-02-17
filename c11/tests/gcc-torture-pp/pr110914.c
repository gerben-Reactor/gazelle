# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110914.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr110914.c" 2


__attribute__ ((noipa)) int
foo (const char *s, unsigned long l)
{
  unsigned char r = 0;
  __builtin_memcpy (&r, s, l != 0);
  return r;
}

int
main ()
{
  const char *p = "123456";
  int a = foo (p, __builtin_strlen (p) - 5);
  int b = foo (p, __builtin_strlen (p) - 6);
  if (a != '1')
    __builtin_abort ();
  if (b != 0)
    __builtin_abort ();
  return 0;
}
