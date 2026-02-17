# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/loop-10.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/loop-10.c" 2


void abort (void);

static int count = 0;

static void
inc (void)
{
  count++;
}

int
main (void)
{
  int iNbr = 1;
  int test = 0;
  while (test == 0)
    {
      inc ();
      if (iNbr == 0)
        break;
      else
        {
          inc ();
          iNbr--;
        }
      test = 1;
    }
  if (count != 2)
    abort ();
  return 0;
}
