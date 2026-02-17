# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990413-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/990413-2.c" 2



extern void abort (void);

static __inline double
minus_zero (void)
{
  union { double __d; int __i[2]; } __x;
  __x.__i[0] = 0x0;
  __x.__i[1] = 0x80000000;
  return __x.__d;
}

static __inline long double
__atan2l (long double __y, long double __x)
{
  register long double __value;
  __asm __volatile__ ("fpatan\n\t"
        : "=t" (__value)
        : "0" (__x), "u" (__y)
        : "st(1)");
  return __value;
}

static __inline long double
__sqrtl (long double __x)
{
  register long double __result;
  __asm __volatile__ ("fsqrt" : "=t" (__result) : "0" (__x));
  return __result;
}

static __inline double
asin (double __x)
{
  return __atan2l (__x, __sqrtl (1.0 - __x * __x));
}

int
main (void)
{
  double x;

  x = minus_zero();
  x = asin (x);

  if (x != 0.0)
    abort ();
  return 0;
}
