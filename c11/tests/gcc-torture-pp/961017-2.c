# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/961017-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/961017-2.c" 2
void abort (void);
void exit (int);

int
main (void)
{
  int i = 0;


  if (sizeof (unsigned long int) == 4)
    {
      unsigned long int z = 0;

      do {
 z -= 0x00004000;
 i++;
 if (i > 0x00040000)
   abort ();
      } while (z > 0);
      exit (0);
    }
  else if (sizeof (unsigned int) == 4)
    {
      unsigned int z = 0;

      do {
 z -= 0x00004000;
 i++;
 if (i > 0x00040000)
   abort ();
      } while (z > 0);
      exit (0);
    }
  else
    exit (0);
}
