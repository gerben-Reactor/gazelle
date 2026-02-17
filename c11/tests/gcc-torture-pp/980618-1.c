# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980618-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/980618-1.c" 2
void abort (void);
void exit (int);

void func(int, int);

int main()
{
        int x = 7;
        func(!x, !7);
 exit (0);
}

void func(int x, int y)
{
        if (x == y)
                return;
        else
                abort ();
}
