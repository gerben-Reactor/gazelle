# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr42544.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr42544.c" 2


extern void abort (void);

int
main ()
{
  signed short s = -1;
  if (sizeof (long long) == sizeof (unsigned int))
    return 0;
  if ((unsigned int) s >= 0x100000000ULL)
    abort ();
  return 0;
}
