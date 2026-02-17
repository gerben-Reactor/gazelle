# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bf-sign-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bf-sign-2.c" 2
# 16 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bf-sign-2.c"
void abort (void);
void exit (int);

struct X {
  unsigned int u3:3;
    signed long int s31:31;
    signed long int s32:32;
  unsigned long int u31:31;
  unsigned long int u32:32;
  unsigned long long ull3 :3;
  unsigned long long ull35:35;
  unsigned u15:15;
};

struct X x;

int
main (void)
{
  if ((x.u3 - 2) >= 0)
    abort ();

  if ((x.s31 - 2) >= 0)
    abort ();

  if ((x.s32 - 2) >= 0)
    abort ();

  if ((x.u15 - 2) >= 0)
    abort ();



  if (sizeof (int) >= 4)
    {
      if ((x.u31 - 2) >= 0)
 abort ();
    }
  else
    {
      if ((x.u31 - 2) < 0)
 abort ();
    }

  if ((x.u32 - 2) < 0)
    abort ();

  if ((x.ull3 - 2) >= 0)
    abort ();

  if ((x.ull35 - 2) < 0)
    abort ();

  exit (0);
}
