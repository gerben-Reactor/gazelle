# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/nestfunc-4.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/nestfunc-4.c" 2







void abort (void);
void exit (int);

long level = 0;
extern long foo (void);
extern long bar (void);







int
main (void)
{
  if (foo () == -42)
    exit (0);

  abort ();
}

long
foo (void)
{
  long tmp = ++level;
  return bar () + tmp;
}

long
bar (void)
{
  long tmp = level;
  return tmp > 500 - 1 ? -42 - tmp : foo () - tmp;
}
