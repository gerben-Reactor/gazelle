# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991014-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991014-1.c" 2
typedef long unsigned int Size_t;
void abort(void);







struct huge_struct
{
  short buf[((1L << (8 * sizeof(Size_t) - 2))-256)];
  int a;
  int b;
  int c;
  int d;
};

union huge_union
{
  int a;
  char buf[((1L << (8 * sizeof(Size_t) - 2))-256)];
};

Size_t union_size()
{
  return sizeof(union huge_union);
}

Size_t struct_size()
{
  return sizeof(struct huge_struct);
}

Size_t struct_a_offset()
{
  return (Size_t)(&((struct huge_struct *) 0)->a);
}

int main()
{

  if (union_size() != sizeof(char) * ((1L << (8 * sizeof(Size_t) - 2))-256))
    abort();

  if (struct_size() != sizeof(short) * ((1L << (8 * sizeof(Size_t) - 2))-256) + 4*sizeof(int))
    abort();

  if (struct_a_offset() < sizeof(short) * ((1L << (8 * sizeof(Size_t) - 2))-256))
    abort();

  return 0;
}
