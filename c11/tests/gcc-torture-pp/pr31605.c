# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr31605.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr31605.c" 2
void abort (void);
void exit (int);

void put_field (unsigned int start, unsigned int len)
{
  int cur_bitshift = ((start + len) % 8) - 8;
  if (cur_bitshift > -8)
    exit (0);
}

int
main ()
{
  put_field (0, 1);
  abort ();
}
