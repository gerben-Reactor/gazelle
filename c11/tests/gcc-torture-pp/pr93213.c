# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93213.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr93213.c" 2





typedef unsigned short u16;
typedef unsigned int u32;
typedef unsigned __int128 u128;

static inline u128
foo (u16 u16_1, u32 u32_1, u128 u128_1)
{

  u128 u128_0 = 0;
  u128_1 -= __builtin_mul_overflow (u32_1, u16_1, &u32_1);
  __builtin_memmove (&u16_1, &u128_0, 2);
  __builtin_memmove (&u16_1, &u128_1, 1);
  return u16_1;



}

__attribute__ ((noipa)) void
bar (void)
{
  char a[] = { 1, 2 };
  const char b[] = { 0, 0 };
  const char c[] = { 2 };
  __builtin_memcpy (a, b, 2);



  __builtin_memcpy (a, c, 1);

  volatile char *p = a;
  if (p[0] != 2 || p[1] != 0)
    __builtin_abort ();
}

int
main (void)
{
  u16 x = foo (-1, -1, 0);
  if (x != 0xff)
    __builtin_abort ();

  bar ();
  return 0;
}
