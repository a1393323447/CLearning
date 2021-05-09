#include <stdio.h>

// 一直以来，我们都没有仔细地了解 printf 的作用
// 现在请结合实例中的注释和程序的输出, 初步了解 printf 函数的功能

// I AM NOT DONE

int main() {

    printf("Hello\n");
    printf("1\t2\t3\n4\t5\t6\n");

    printf("50%%\n");

    int interge = 1;
    printf("interge = %i\n", interge);
    printf("interge = %d\n", interge);

    printf("[%4d]\n", interge);
    printf("[%-4d]\n", interge);
    printf("[%04d]\n", interge);

    unsigned int u_num = 107;
    printf("u_num = %u\n", u_num);
    printf("u_num = %d\n", u_num);
    printf("u_num in octonary = %o\n", u_num);
    printf("u_num in hexadecimal = %x\n", u_num);
    printf("u_num in hexadecimal = %X\n", u_num);

    char a = 97;
    printf("a: \"%c\"\n", a);
    printf("a in ASCII: %d\n", a);
    printf("a in ASCII: %u\n", a);
    return 0;
}