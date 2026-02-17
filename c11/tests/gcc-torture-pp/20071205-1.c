# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20071205-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20071205-1.c" 2


extern void abort (void);

int
foo (int x)
{
  return ((x << 8) & 65535) | 255;
}

int
main (void)
{
  if (foo (0x32) != 0x32ff || foo (0x174) != 0x74ff)
    abort ();
  return 0;
}
