# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/struct-ret-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/struct-ret-2.c" 2
void abort (void);
void exit (int);

typedef struct
{
  unsigned char a __attribute__ ((packed));
  unsigned short b __attribute__ ((packed));
} three_byte_t;

unsigned char
f (void)
{
  return 0xab;
}

unsigned short
g (void)
{
  return 0x1234;
}

int
main (void)
{
  three_byte_t three_byte;

  three_byte.a = f ();
  three_byte.b = g ();
  if (three_byte.a != 0xab || three_byte.b != 0x1234)
    abort ();
  exit (0);
}
