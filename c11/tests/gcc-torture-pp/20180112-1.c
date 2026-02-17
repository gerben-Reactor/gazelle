# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20180112-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20180112-1.c" 2



extern void abort (void);

typedef unsigned int u32;

u32 bug (u32 * result) __attribute__((noinline));
u32 bug (u32 * result)
{
  volatile u32 ss = 0xFFFFffff;
  volatile u32 d = 0xEEEEeeee;
  u32 tt = d & 0x00800000;
  u32 r = tt << 8;

  r = (r >> 31) | (r << 1);

  u32 u = r^ss;
  u32 off = u >> 1;

  *result = tt;
  return off;
}

int main(void)
{
  u32 l;
  u32 off = bug(&l);
  if (off != 0x7fffffff)
    abort ();
  return 0;
}
