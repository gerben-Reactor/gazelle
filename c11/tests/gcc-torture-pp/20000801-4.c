# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000801-4.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000801-4.c" 2







extern void abort (void);
extern void exit (int);

int
foo (void)
{
  char s[2] = "";
  return 0 == s[1];
}

char *t;

int
main (void)
{
  {
    char s[] = "x";
    t = s;
  }
  if (foo ())
    exit (0);
  else
    abort ();
}
