# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000412-5.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20000412-5.c" 2
void abort (void);
void exit (int);

int main( void ) {
    struct {
 int node;
 int type;
    } lastglob[1] = { { 0 , 1 } };

    if (lastglob[0].node != 0 || lastglob[0].type != 1)
      abort ();
    exit (0);
}
