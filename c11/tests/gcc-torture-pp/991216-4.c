# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991216-4.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/991216-4.c" 2

void abort (void);
void exit (int);

static void bug(int size, int tries)
{
    int i;
    int num = 0;
    while (num < size)
    {
        for (i = 1; i < tries; i++) num++;
    }
}

int main()
{
    bug(5, 10);
    exit (0);
}
