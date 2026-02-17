# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020529-1.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020529-1.c" 2
# 15 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/20020529-1.c"
void abort (void);
void exit (int);

struct xx
 {
   int a;
   struct xx *b;
   short c;
 };

int f1 (struct xx *);
void f2 (void);

int
foo (struct xx *p, int b, int c, int d)
{
  int a;

  for (;;)
    {
      a = f1(p);
      if (a)
 return (0);
      if (b)
 continue;
      p->c = d;
      if (p->a)
 f2 ();
      if (c)
 f2 ();
      d = p->c;
      switch (a)
 {
 case 1:
   if (p->b)
     f2 ();
   if (c)
     f2 ();
 default:
   break;
 }
    }
  return d;
}

int main (void)
{
  struct xx s = {0, &s, 23};
  if (foo (&s, 0, 0, 0) != 0 || s.a != 0 || s.b != &s || s.c != 0)
    abort ();
  exit (0);
}

int
f1 (struct xx *p)
{
  static int beenhere = 0;
  if (beenhere++ > 1)
    abort ();
  return beenhere > 1;
}

void
f2 (void)
{
  abort ();
}
