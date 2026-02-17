# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20091229-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20091229-1.c" 2
void abort (void);
void exit (int);

long long foo(long long v) { return v / -0x080000000LL; }
int main(int argc, char **argv) { if (foo(0x080000000LL) != -1) abort(); exit (0); }
