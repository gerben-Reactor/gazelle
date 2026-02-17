# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/arith-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/arith-1.c" 2
void abort (void);
void exit (int);

unsigned
sat_add (unsigned i)
{
  unsigned ret = i + 1;
  if (ret < i)
    ret = i;
  return ret;
}

unsigned
sat_add2 (unsigned i)
{
  unsigned ret = i + 1;
  if (ret > i)
    return ret;
  return i;
}

unsigned
sat_add3 (unsigned i)
{
  unsigned ret = i - 1;
  if (ret > i)
    ret = i;
  return ret;
}

unsigned
sat_add4 (unsigned i)
{
  unsigned ret = i - 1;
  if (ret < i)
    return ret;
  return i;
}

int
main (void)
{
  if (sat_add (~0U) != ~0U)
    abort ();
  if (sat_add2 (~0U) != ~0U)
    abort ();
  if (sat_add3 (0U) != 0U)
    abort ();
  if (sat_add4 (0U) != 0U)
    abort ();
  exit (0);
}
