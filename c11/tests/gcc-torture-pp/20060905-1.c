# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20060905-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20060905-1.c" 2



extern void abort(void);

volatile char s[256][3];

char g;

static void dummy(char a)
{
  g = a;
}

static int foo(void)
{
  int i, j=0;

  for (i = 0; i < 256; i++)
    if (i >= 128 && i < 256)
      {
 dummy (s[i - 128][0]);
 ++j;
      }

  return j;
}

int main(void)
{
  if (foo () != 128)
    abort ();

  return 0;
}
