# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921204-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921204-1.c" 2


void abort (void);
void exit (int);
# 13 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/921204-1.c"
typedef struct {
  unsigned b0:1, f1:17, b18:1, b19:1, b20:1, f2:11;
} bf;

typedef union {
  bf b;
  unsigned w;
} bu;

bu
f(bu i)
{
  bu o = i;

  if (o.b.b0)
    o.b.b18 = 1,
    o.b.b20 = 1;
  else
    o.b.b18 = 0,
    o.b.b20 = 0;

  return o;
}

int
main(void)
{
  bu a;
  bu r;

  a.w = 0x4000000;
  a.b.b0 = 0;
  r = f(a);
  if (a.w != r.w)
    abort();
  exit(0);
}
