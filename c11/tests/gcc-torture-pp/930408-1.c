# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930408-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/930408-1.c" 2
void abort (void);
void exit (int);

typedef enum foo E;
enum foo { e0, e1 };

struct {
  E eval;
} s;

void
p(void)
{
  abort();
}

void
f(void)
{
  switch (s.eval)
    {
    case e0:
      p();
    }
}

int
main(void)
{
  s.eval = e1;
  f();
  exit(0);
}
