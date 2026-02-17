# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921202-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921202-2.c" 2
void abort (void);
void exit (int);

int
f(long long x)
{
  x >>= 8;
  return x & 0xff;
}

int
main(void)
{
  if (f(0x0123456789ABCDEFLL) != 0xCD)
    abort();
  exit (0);
}
