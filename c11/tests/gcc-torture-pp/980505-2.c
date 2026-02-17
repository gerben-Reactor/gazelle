# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980505-2.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980505-2.c" 2
void abort (void);
void exit (int);

typedef unsigned short Uint16;
typedef unsigned int Uint;

Uint f ()
{
        Uint16 token;
        Uint count;
        static Uint16 values[1] = {0x9300};

        token = values[0];
        count = token >> 8;

        return count;
}

int
main ()
{
  if (f () != 0x93)
    abort ();
  exit (0);
}
