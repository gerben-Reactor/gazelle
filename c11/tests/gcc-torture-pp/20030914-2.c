# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030914-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030914-2.c" 2





void abort (void);
void exit (int);

struct s { int i[18]; };

int f (struct s pa, int pb, ...)
{
  return pb;
}

struct s gs;

int main ()
{
  if (f (gs, 0x1234) != 0x1234)
    abort ();

  exit (0);
}
