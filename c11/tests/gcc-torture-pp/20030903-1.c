# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030903-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20030903-1.c" 2




void abort (void);

enum X { X1 = 1, X2, X3, X4 };
static volatile enum X test = 0;
static void y(int);

int main()
{
  switch (test)
    {
    case X1: y(1); break;
    case X2: y(2); break;
    case X3: y(3); break;
    case X4: y(4); break;
    }
  return 0;
}

static void y(int x) { abort (); }
