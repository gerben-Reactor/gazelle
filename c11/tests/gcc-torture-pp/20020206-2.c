# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020206-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020206-2.c" 2





typedef struct { unsigned short a; } A;

extern void abort (void);
extern void exit (int);

void foo (unsigned int x)
{
  if (x != 0x800 && x != 0x810)
    abort ();
}

int
main (int argc, char **argv)
{
  int i;
  for (i = 0; i < 2; ++i)
    foo (((A) { ((!(i >> 4) ? 8 : 64 + (i >> 4)) << 8) + (i << 4) } ).a);
  exit (0);
}
