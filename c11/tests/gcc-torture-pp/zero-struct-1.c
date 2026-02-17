# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/zero-struct-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/zero-struct-1.c" 2
struct g{};
char y[3];
char *f = &y[0];
char *ff = &y[0];
void h(void)
{
  struct g t;
  *((struct g*)(f++)) = *((struct g*)(ff++));
  *((struct g*)(f++)) = (struct g){};
  t = *((struct g*)(ff++));
}

void abort (void);

int main(void)
{
  h();
  if (f != &y[2])
    abort();
  if (ff != &y[2])
    abort();
  return 0;
}
