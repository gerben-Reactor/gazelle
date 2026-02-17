# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/nestfunc-5.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/nestfunc-5.c" 2


extern void abort (void);
extern void exit (int);

static void recursive (int n, void (*proc) (void))
{
  __label__ l1;

  void do_goto (void)
  {
    goto l1;
  }

  if (n == 3)
      recursive (n - 1, do_goto);
  else if (n > 0)
    recursive (n - 1, proc);
  else
    (*proc) ();
  return;

l1:
  if (n == 3)
    exit (0);
  else
    abort ();
}

int main ()
{
  recursive (10, abort);
  abort ();
}
