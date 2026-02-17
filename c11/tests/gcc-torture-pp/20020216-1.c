# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020216-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020216-1.c" 2




extern void abort ();
extern void exit (int);

signed char c = (signed char) 0xffffffff;

int foo (void)
{
  return (unsigned short) c ^ (signed char) 0x99999999;
}

int main (void)
{
  if ((unsigned char) -1 != 0xff
      || sizeof (short) != 2
      || sizeof (int) != 4)
    exit (0);
  if (foo () != (int) 0xffff0066)
    abort ();
  exit (0);
}
