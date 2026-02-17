# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030914-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030914-1.c" 2




void abort (void);
void exit (int);

struct s { int val[16]; };

long double f (int pa, struct s pb, long double pc)
{
  int i;

  for (i = 0; i < 16; i++)
    pc += pb.val[i];
  return pc;
}

int main ()
{
  struct s x;
  int i;

  for (i = 0; i < 16; i++)
    x.val[i] = i + 1;
  if (f (1, x, 10000.0L) != 10136.0L)
    abort ();
  exit (0);
}
