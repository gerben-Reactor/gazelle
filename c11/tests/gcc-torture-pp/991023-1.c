# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991023-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991023-1.c" 2
void abort (void);
void exit (int);

int blah;

int
foo(void)
{
  int i;

  for (i=0 ; i< 7 ; i++)
    {
      if (i == 7 - 1)
 blah = 0xfcc;
      else
 blah = 0xfee;
    }
  return blah;
}

int
main(void)
{
  if (foo () != 0xfcc)
    abort ();
  exit (0);
}
