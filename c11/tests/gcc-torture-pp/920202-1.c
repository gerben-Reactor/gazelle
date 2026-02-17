# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920202-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/920202-1.c" 2
void exit (int);

static int rule_text_needs_stack_pop = 0;
static int input_stack_pos = 1;

int
f (void)
{
  rule_text_needs_stack_pop = 1;

  if (input_stack_pos <= 0)
    return 1;
  else
    return 0;
}

int
main (void)
{
  f ();
  exit (0);
}
