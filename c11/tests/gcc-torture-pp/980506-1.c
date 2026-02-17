# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980506-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980506-1.c" 2
void exit (int);

struct decision
{
  char enforce_mode;
  struct decision *next;
};


static void
clear_modes (p)
     register struct decision *p;
{
  goto blah;

foo:
  p->enforce_mode = 0;
blah:
  if (p)
    goto foo;
}

int
main(void)
{
  struct decision *p = 0;
  clear_modes (p);
  exit (0);
}
