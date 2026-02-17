# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/bitfld-2.c" 2


extern void abort (void);
extern void exit (int);

struct A {
  unsigned short a : 5;
  unsigned short b : 5;
  unsigned short c : 6;
};

struct B {
  unsigned short a : 5;
  unsigned short b : 3;
  unsigned short c : 8;
};

int main ()
{

  if ((unsigned short) 65521 != 65521)
    exit (0);

  if (sizeof (struct A) != sizeof (struct B))
    abort ();

  exit (0);
}
