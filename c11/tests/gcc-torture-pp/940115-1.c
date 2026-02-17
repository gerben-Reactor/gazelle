# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/940115-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/940115-1.c" 2

f (cp, end)
     char *cp;
     char *end;
{
  return (cp < end);
}

main ()
{
  if (! f ((char *) 0, (char *) 1))
    abort();
  exit (0);
}
