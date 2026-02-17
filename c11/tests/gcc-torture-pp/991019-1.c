# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991019-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991019-1.c" 2
void abort (void);
void exit (int);

typedef struct {
     double epsilon;
} material_type;

material_type foo(double x)
{
     material_type m;

     m.epsilon = 1.0 + x;
     return m;
}

int
main(void)
{
  int i;
  material_type x;



  for (i = 0; i < 10; i++)
    {
      x = foo (1.0);
      if (x.epsilon != 1.0 + 1.0)
 abort ();
    }

  exit (0);
}
