// 为了让程序可以表示现实世界中繁多的事物, C语言为我们内置了许多基础类型
// 在这次练习中, 我们会学着使用 int 类型说明符, 声明一些 int 类型的变量

// int 是 integer (整数) 的缩写
// 使用 int 可以声明一个变量的类型是一个整数

// I AM NOT DONE

#include <stdio.h>

int main() {

    int a;
    a = 1;
    printf("a = %d", a);

    /*类型名*/ /*标识符*/;
    /*标识符*/ = /*任意一个整数*/;
    printf("你的标识符 = %d", /*你的标识符*/);
}

// 注解: 当你输入一个错误的标识符时, 会有编译错误
//       标识符的命名规则详见 exercise/C语言程序基础结构/标识符.c
//
//       当你将一个不在 [-2147483648, 2147483647] 区间内的整数时
//       结果并不会如你所愿, 原因是: 发生了溢出, 至于什么是溢出,
//       详见: 