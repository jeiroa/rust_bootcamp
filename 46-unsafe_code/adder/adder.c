// compile with:
// > gcc adder/adder.c -o adder/adder.o -c
// > ar -rc ./adder/libadder.a ./adder/adder.o
int add(int a, int b)
{
    return a + b;
}