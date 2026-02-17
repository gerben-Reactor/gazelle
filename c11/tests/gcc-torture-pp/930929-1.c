# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930929-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930929-1.c" 2


sub1 (i)
     int i;
{
  return i - (5 - i);
}

sub2 (i)
     int i;
{
  return i + (5 + i);
}

sub3 (i)
     int i;
{
  return i - (5 + i);
}

sub4 (i)
     int i;
{
  return i + (5 - i);
}

main()
{
  if (sub1 (20) != 35)
    abort ();
  if (sub2 (20) != 45)
    abort ();
  if (sub3 (20) != -5)
    abort ();
  if (sub4 (20) != 5)
    abort ();
  exit (0);
}
