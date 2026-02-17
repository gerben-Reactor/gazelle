# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/switch-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/switch-1.c" 2







extern void abort (void);

int
foo (int x)
{
  switch (x)
    {
    case 4:
    case 6:
    case 9:
    case 11:
      return 30;
    }
  return 31;
}

int
main ()
{
  int i, r;

  for (i=-1; i<66; i++)
    {
      r = foo (i);
      if (i == 4)
 {
   if (r != 30)
     abort ();
 }
      else if (i == 6)
 {
   if (r != 30)
     abort ();
 }
      else if (i == 9)
 {
   if (r != 30)
     abort ();
 }
      else if (i == 11)
 {
   if (r != 30)
     abort ();
 }
      else if (r != 31)
 abort ();
    }
  return 0;
}
