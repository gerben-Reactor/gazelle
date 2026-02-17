# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr58640.c"
# 1 "<built-in>" 1
# 1 "<built-in>" 3
# 389 "<built-in>" 3
# 1 "<command line>" 1
# 1 "<built-in>" 2
# 1 "/home/priv-gerben/project/gazelle/c11/tests/gcc-src/gcc/testsuite/gcc.c-torture/execute/pr58640.c" 2
void exit (int);

int a, b, c, d = 1, e;

static signed char
foo ()
{
  int f, g = a;

  for (f = 1; f < 3; f++)
    for (; b < 1; b++)
      {
        if (d)
          for (c = 0; c < 4; c++)
            for (f = 0; f < 3; f++)
              {
                for (e = 0; e < 1; e++)
                  a = g;
                if (f)
                  break;
              }
        else if (f)
          continue;
        return 0;
      }
  return 0;
}

int
main ()
{
  foo ();
  exit (0);
}
