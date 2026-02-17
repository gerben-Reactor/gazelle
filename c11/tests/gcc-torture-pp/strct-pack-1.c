# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/strct-pack-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/strct-pack-1.c" 2
void abort (void);
void exit (int);

typedef struct
{
  short s __attribute__ ((aligned(2), packed));
  double d __attribute__ ((aligned(2), packed));
} TRIAL;

int
check (TRIAL *t)
{
  if (t->s != 1 || t->d != 16.0)
    return 1;
  return 0;
}

int
main (void)
{
  TRIAL trial;

  trial.s = 1;
  trial.d = 16.0;

  if (check (&trial) != 0)
    abort ();
  exit (0);
}
