# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20021011-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20021011-1.c" 2


extern void abort (void);

char buf[64];

int
main (void)
{
  int i;

  __builtin_strcpy (buf, "mystring");
  if (__builtin_strcmp (buf, "mystring") != 0)
    abort ();

  for (i = 0; i < 16; ++i)
    {
      __builtin_strcpy (buf + i, "mystring");
      if (__builtin_strcmp (buf + i, "mystring") != 0)
 abort ();
    }

  return 0;
}
