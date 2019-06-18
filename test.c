int expect(char *name, int expected, int actual) {
    if (expected == actual) {
        printf("%s => %d\n", name, actual);
    } else {
        printf("Expected %d, but got %d\n", expected, actual);
    }
    return 0;
}

int plus(int a, int b) {
    return a + b;
}

int end(int arr[5]) {
    return arr[4];
}

int global_var;
int global_a = 10;
int *global_b = &global_a;

int main() {
    expect("zero", 0, 0);
    expect("one", 1, 1);
    expect("-1", -1, -1);
    expect("-2", -2, 0 - 2);

    expect("add", 3, 1 + 2);
    expect("sub", 10, 84 - 74);
    expect("mul", 24, 3 * 8);
    expect("div", 5, 25 / 5);
    expect("div2", 5, 28 / 5);
    expect("precedence", 19, 4 + 5 * 3);
    expect("precedence2", 11, 5 + (7 - 5) * 3);

    int a;
    a = 5;
    expect("variable", 5, a);
    a = a * 4;
    expect("variable2", 20, a);
    if (0)
        a = 3;
    expect("if", 20, a);
    if (1)
        a = 5;
    expect("if", 5, a);

    expect("less than", 0, 5 < 4);
    expect("less than 2", 0, 5 < 5);
    expect("greater than", 1, 6 > 5);
    expect("greater than or equal", 1, 6 >= 6);
    expect("equal", 1, 5 == 5);
    expect("not equal", 1, 5 != 4);

    int b;
    b = 0;
    while (b < 5) {
        b = b + 1;
    }
    expect("while", 5, b);
    int i;
    for (i = 0; i < 10; i = i + 1) {
    }
    expect("for", 10, i);

    int *c;
    c = &a;
    *c = 10;
    expect("pointer", 10, a);
    int **d;
    d = &c;
    **d = 20;
    expect("double pointer", 20, a);

    int arr[5];
    arr[0] = 4;
    arr[1] = 7;
    arr[4] = 10;
    expect("array", 4, arr[0]);
    expect("pointer calculation", 7, *(arr + 1));

    expect("call", 45, plus(10, 35));
    expect("call2", 30, plus(5, 4) + plus(5, 16));
    expect("call3", 10, end(arr));

    char ch;

    expect("sizeof", 4, sizeof(a));
    expect("sizeof pointer", 8, sizeof c);
    expect("sizeof char", 1, sizeof(ch));
    expect("sizeof array", 20, sizeof(arr));

    global_var = 14;
    expect("global variable", 14, global_var);
    expect("global_a = 10", 10, global_a);
    *global_b = 20;
    expect("global_b = &global_a", 20, global_a);

    char *text;
    text = "hello";
    expect("string literal", 108, text[3]);

    expect("complex calculation", 128, ((((((1+1)+(1+1))+(1+1)+(1+1))+(((1+1)+(1+1))+(1+1)+(1+1)))+((((1+1)+(1+1))+(1+1)+(1+1))+(((1+1)+(1+1))+(1+1)+(1+1))))+(((((1+1)+(1+1))+(1+1)+(1+1))+(((1+1)+(1+1))+(1+1)+(1+1)))+((((1+1)+(1+1))+(1+1)+(1+1))+(((1+1)+(1+1))+(1+1)+(1+1)))))+((((((1+1)+(1+1))+(1+1)+(1+1))+(((1+1)+(1+1))+(1+1)+(1+1)))+((((1+1)+(1+1))+(1+1)+(1+1))+(((1+1)+(1+1))+(1+1)+(1+1))))+(((((1+1)+(1+1))+(1+1)+(1+1))+(((1+1)+(1+1))+(1+1)+(1+1)))+((((1+1)+(1+1))+(1+1)+(1+1))+(((1+1)+(1+1))+(1+1)+(1+1))))));

    return 0;
}