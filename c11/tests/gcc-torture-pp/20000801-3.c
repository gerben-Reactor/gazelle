# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000801-3.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000801-3.c" 2




void abort (void);
void exit (int);

typedef struct { } empty;

typedef struct {
  int i;
  empty e;
  int i2;
} st;

st s = { .i = 0, .i2 = 1 };

extern void abort (void);

int
main (void)
{
  if (s.i2 == 1)
    exit (0);
  else
    abort ();
}
