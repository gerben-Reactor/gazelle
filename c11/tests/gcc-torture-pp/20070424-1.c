# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20070424-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20070424-1.c" 2
extern void abort (void);
extern void exit (int);

void do_exit (void) { exit (0); }
void do_abort (void) { abort (); }

void foo (int x, int a)
{
  if (x < a)
    goto doit;
  do_exit ();
  if (x != a)
    goto doit;


  do_abort ();
  return;

doit:
  do_abort ();
}

int main()
{
  foo (1, 0);
  return 0;
}
