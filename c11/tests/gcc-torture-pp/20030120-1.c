# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030120-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030120-1.c" 2



void abort (void);
void exit (int);

unsigned short
test1 (unsigned short w)
{
  if ((w & 0xff00) == 0)
    {
      if (w == 0)
 w = 2;
    }
  return w;
}

unsigned long
test2 (unsigned long w)
{
  if ((w & 0xffff0000) == 0)
    {
      if (w == 0)
 w = 2;
    }
  return w;
}

int
test3 (unsigned short a)
{
  if (a & 1)
    return 1;
  else if (a)
    return 1;
  else
    return 0;
}

int
main ()
{
  if (test1 (1) != 1)
    abort ();

  if (test2 (1) != 1)
    abort ();

  if (test3 (2) != 1)
    abort ();

  exit (0);
}
