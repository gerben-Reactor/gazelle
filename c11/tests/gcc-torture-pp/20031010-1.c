# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20031010-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20031010-1.c" 2


void abort (void);

long __attribute__((noinline))
foo (long ct, long cf, _Bool p1, _Bool p2, _Bool p3)
{
  long diff;

  diff = ct - cf;

  if (p1)
    {
      if (p2)
 {
   if (p3)
     {
       long tmp = ct;
       ct = cf;
       cf = tmp;
     }
   diff = ct - cf;
 }

      return diff;
    }

  abort ();
}

int main ()
{
  if (foo(2, 3, 1, 1, 1) == 0)
    abort ();
  return 0;
}
