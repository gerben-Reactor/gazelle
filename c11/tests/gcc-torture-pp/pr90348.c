# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr90348.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr90348.c" 2


void __attribute__ ((noipa))
set_one (unsigned char *ptr)
{
  *ptr = 1;
}

void __attribute__ ((noipa))
check_zero (unsigned char const *in, unsigned int len)
{
  for (unsigned int i = 0; i < len; ++i)
    if (in[i] != 0)
      __builtin_abort ();
}

static void
set_one_on_stack (void)
{
  unsigned char buf[1];
  set_one (buf);
}

int
main ()
{
  for (int i = 0; i <= 4; ++i)
    {
      unsigned char in[4];
      for (int j = 0; j < i; ++j)
 {
   in[j] = 0;
   set_one_on_stack ();
 }
      check_zero (in, i);
    }
  return 0;
}
