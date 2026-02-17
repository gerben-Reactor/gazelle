# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr114965.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr114965.c" 2


static void
foo (const char *x)
{

  char a = '0';
  while (1)
    {
      switch (*x)
 {
 case '_':
 case '+':
   a = *x;
   x++;
   continue;
 default:
   break;
 }
      break;
    }
  if (a == '0' || a == '+')
    __builtin_abort ();
}

int
main ()
{
  foo ("_");
}
