// 标识符说的通俗一点, 其实就是一个名字
// 名字嘛, 很好理解. 就是用来指代一样东西
// 这次练习中, 先来学习怎么样 命名

/*
    标识符是数字、下划线、小写及大写拉丁字母和以 \u 及 \U 转义记号指定的 Unicode 字符 (C99 起)的任意长度序列。
    合法的标识符必须以非数字字符（拉丁字母、下划线或 Unicode 非数字字符 (C99 起)）开始。
    标识符大小写有别（小写和大写字母不同）。
*/

// 练习: 填补并改正下面的代码, 使得文件通过编译
// 在终端输入 hint 获取提示 

// I AM NOT DONE

#include <stdio.h>

int main() {

    int /*请给你的变量命名*/ = 1;
    printf("你命名的变量 = %d", /*填入你的变量的名字*/);

    // 请参照命名标识符的规则, 改正下列代码
    int 3a = 1;
    int %a = 2;
    int -a = 3;
    printf("first = %d\nsecond = %d\nthird = %d\n", 3a, %a, -a);

    int b = 2;
    printf("b = %d", B);

    return 0;
}