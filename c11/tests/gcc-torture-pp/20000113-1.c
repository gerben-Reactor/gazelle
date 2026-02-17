# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000113-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000113-1.c" 2
void abort (void);
void exit (int);

struct x {
  unsigned x1:1;
  unsigned x2:2;
  unsigned x3:3;
};

void
foobar (int x, int y, int z)
{
  struct x a = {x, y, z};
  struct x b = {x, y, z};
  struct x *c = &b;

  c->x3 += (a.x2 - a.x1) * c->x2;
  if (a.x1 != 1 || c->x3 != 5)
    abort ();
  exit (0);
}

int
main(void)
{
  foobar (1, 2, 3);
}
