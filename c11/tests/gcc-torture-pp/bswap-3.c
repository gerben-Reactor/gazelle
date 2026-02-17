# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bswap-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bswap-3.c" 2


int f(unsigned int a) __attribute__((noipa));
int f(unsigned int a)
{
  return ((__builtin_bswap32(a))>>24) & 0x3;
}


int g(unsigned int a) __attribute__((noipa));
int g(unsigned int a)
{
  return a&0x3;
}

int main(void)
{
  for (int b = 0; b <= 0xF; b++)
    {
      if (f(b) != g(b))
 __builtin_abort ();
    }
  return 0;
}
