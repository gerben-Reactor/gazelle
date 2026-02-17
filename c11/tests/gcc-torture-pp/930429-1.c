# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930429-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930429-1.c" 2
void abort (void);
void exit (int);

char *
f (char *p)
{
  short x = *p++ << 16;
  return p;
}

int
main (void)
{
  char *p = "";
  if (f (p) != p + 1)
    abort ();
  exit (0);
}
