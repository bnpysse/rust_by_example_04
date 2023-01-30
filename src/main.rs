#![allow(overflowing_literals)]
#![allow(unreachable_code)]
fn main() {
    // 2023年1月27日23时18分20秒
    // Rust_by_example_04,第四章的学习.

    // 4.变量绑定
    println!("=====4.变量绑定=====");
    // Rust 通过静态类型确保类型安全
    //      变量绑定时可以在声明时说明类型,不过,大多数情况下,编译器能够从上下文推导出变量的类型,从而大大减少了类型说明的工作
    // 使用 let 绑定操作可以将值(比如字面量) 绑定 (bind) 到变量
    let an_integer = 1_u32;
    let a_boolean = true;
    let unit = ();

    // 将 an_integer 复制到 copied_integer
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用的变量绑定产生警告,可以给变量名加上 下划线前缀 来消除警告
    let _unused_variable = 3_u32;
    let _nosisy_unused_variable = 2u32;
    
    // 4.1.可变变量
    println!("\n\n=====4.1.可变变量=====");
    // 变量绑定默认是不可变的 (immutable), 但加上 mut 修饰语后变量就可以改变
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
    println!("Before mutation: {}", mutable_binding);
    // 正确的代码
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);
    // 错误代码
    // _immutable_binding += 1;

    // 4.2.作用域和遮蔽
    println!("\n\n=====4.2.作用域和遮蔽=====");
    // 变量绑定有一个作用域(scope),它被限定只在一个代码块(block)中生存(live)
    // 代码块是一个被 {} 包围的语句集合
    // 另外,也允许 变量遮蔽(variable shdowing)
    // 此绑定生存于 main 函数中
    let long_lived_binding = 1;
    {
        // 这是一个代码块, 比main函数拥有更小的作用域
        let short_lived_binding = 2;
        println!("inner short: {}", short_lived_binding);

        // 此绑定 遮蔽 了外面的绑定
        let long_lived_binding = 5_f32;
        println!("inner long: {}", long_lived_binding);
    }   //代码块结束 
    // Error!! short_lived_binding 在此作用域上已经不存在了
    // println!("outer long: {}", short_lived_binding);

    println!("outer long: {}", long_lived_binding);

    // 绑定同样 遮蔽 了前面的绑定
    let long_lived_binding = 'a';

    println!("outer long: {}", long_lived_binding);

    // 4.3.变量先声明
    println!("\n\n=====4.3.变量先声明=====");
    // 声明一个变量绑定
    let a_binding;
    {
        let x = 2;
        a_binding = x * x;
    }
    println!("a binding: {}", a_binding);

    let another_binding;
    // 是下面这句出错,因为没有绑定变量 another_binding
    // println!("another binding: {}", another_binding);
    another_binding = 1;
    println!("another binding: {}", another_binding);

    // 4.4.冻结
    println!("\n\n=====4.4.冻结=====");
    // 当数据被相同的名称不变地绑定时,它还会 冻结('freeze'),
    // 在不可弯绑定超作用域之前,无法修改已冻结的数据
    let mut _mutable_integer = 7i32;
    {
        // 被不可变的 _mutable_integer 遮蔽
        let _mutable_integer = _mutable_integer;
        // 报错!!!
        // _mutable_integer = 50;
        // _mutable_integer 离开作用域
    }
    // 正常运行! _mutable_integer 在空虚作用域没有被冻结
    _mutable_integer = 3;

    // 5.类型系统
    println!("\n\n=====5.类型系统=====");
    // Rust 提供多种机制,用于改变或定义原生类型和用户定义类型
    //      原生类型的类型转换(cast)
    //      指定字面量的类型
    //      使用类型推断(type inference)
    //      给类型取别名(alias)

    // 5.1.类型转换
    println!("\n\n=====5.1.类型转换=====");
    // Rust 不提供原生类型之间的隐式类型转换(coercion),但可以使用 as 关键字进行显式类型转换(casting)
    // 整形之间的转换大体遵循C语言的惯例,除了C会产生未定义行为的情形
    // 在 Rust 中 所有 整形转换都是定义良好的
    let decimal = 65.4321_f32;
    // 错误!!!!!!!!
    // let integer: u8 = decimal;

    // 但可以显式转换
    let integer = decimal as u8;
    let character = integer as char;
    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 当把任何类型转换为无符号类型 T 时,会不断加上或减去(std::T::MAX + 1)
    // 直到值位于新类型 T 的范围之内
    println!("1000 as a u16 is: {}", 1000 as u16);
    // 1000 - 256 - 256 - 256 = 232
    // 事实上的处理方式是:从最低有效位(LSB, least significant bits)开始保留
    // 8 位,然后剩余位置,直到最高有效位(MSB, most significant bits)都被抛弃
    // MSB 就是二进制的最高位
    // LSB 就是二进制的最低位
    // 按日常书写习惯即最左边一位和最右边一位
    println!("1000 as a u8 is: {}", 1000 as u8);
    // -1 + 256 = 255
    println!("-1 as a u8 is: {}", (-1i8) as u8);
    // 对正数,这就各取模一样
    println!("1000 mod 256 is: {}", 1000 % 256);
    // 当转换到有符号类型时,(位操作的)结果就和 "先转换到对应的无符号类型,如果 MSB 是 1,
    // 则 该值为负" 是一样的.当然如果数值已经在目标类型的范围内,就直接把它放进去
    println!("128 as a i16 is: {}", 128 as i16);
    // 128转换 u8 还是 128, 但转到 i8 相当于给 128 取八位的二进制补码,其值为:
    println!("128 as a i8 is: {}", 128 as i8);
    // 重复之前的例子
    // 1000 as u8 -> 232
    println!("1000 as a u8 is: {}", 1000 as u8);
    // 232 的二进制补码是 -24
    println!("232 as a i8 is: {}", 232 as i8);

    // 5.2.字面量
    println!("\n\n=====5.2.字面量=====");
    // 带后缀的字面量,其类型在初始化时已经知道了
    let x = 1_u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量,其类型取决于如何使用它们
    let i = 1;
    let f = 1.0;
    
    // size_of_val 返回一个变量所占用的字节数
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
    
    // fun(&foo) 用 传引用(pass by reference)的方式把变量传给函数，而非传值
    // (pass by value)，写法是 fun(foo))
    // std::mem::size_of_val 是一个函数，这里是完整路径调用，代码可以分成一些
    // 叫做模块(module)的逻辑单元。在本例中， size_of_val是在 mem 模块中定义的，
    // 而 mem 模块又是在 std crate 中定义的。

    // 5.3.类型推断
    println!("\n\n=====5.3.类型推断=====");
    // Rust 的类型推断引擎是很聪明的，它不只是在初始化时年看右值 (r-value) 的类型而已，
    // 它还会考察变量之后会怎样使用，借此推断类型。
    
    // 因为有类型说明，编译器知道 elem 的类型是 u8。
    let elem = 5_u8;
    // 创建一个空向量(vector,即不定长的，可以增长的数组)
    let mut vec = Vec::new();
    // 现在编译器还不知道 vec 的具体类型，只知道它是某种东西构成的
    // 向量 (Vec<_>)
    // 在向量中插入 elem
    vec.push(elem);
    // 哈哈，现在编译器就知道 vec 是 u8 的向量了 (`Vec<u8>`>)。
    println!("the vec is: {:?}, the size of vec is: {}", vec, std::mem::size_of_val(&vec));

    // 5.4.别名
    println!("\n\n=====5.4.别名=====");
    // 可以用 type 语句给已有的类型取个新的名字。
    // 类型的名字必须遵循驼峰命名法，否则编译器将给出警告
    // 原生类型除外，比如：usize、f32等等
    // `NanoSecond` 是 `u64` 的新名字
    type NanoSecond = u64;
    type Inch = u64;

    // 通过这个属性屏蔽警告
    #[allow(non_camel_case_types)]
    type u64_t = u64;
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;
    println!("{} nanoseconds + {} inches = {} unit?",
        nanoseconds,
        inches,
        nanoseconds + inches);
    // 别名的主要用途是避免写出冗长的模板化代码(boilerplate code)
    // 如 IoResult<T> 是 Result<T, IoError> 类型的别名

    // 6.类型转换
    println!("\n\n=====6.类型转换=====");
    // Rust 使用 trait 解决类型之间的转换问题。
    // 最一般的转换会用到 From 和 Into 两个trait
    // 不过，即便常见的情况也可能会用到特别的 trait ，尤其是 String 转换到别的类型
    // 以及把别的类型转换到 String 时

    // 6.1.From和Into
    println!("\n\n=====6.1.From和Into=====");
    // From Into两个 trait 是内部相联的，实际上这是它们实现的一部分。
    // 如果我们能够从类型 A 得到类型 B ，那么很容易相信我们也能把类型B转换为类型A
    // From trait允许一种类型定义“怎么根据另一种类型生成自己”，因此它提供了一种
    // 类型转换的简单机制，在标准库中有无数 From 的实现，规定原生类型及其他常见类型的转换
    // 比如，可以 很容易地把 str 转换成 String
    // 为自己的类型定义转换机制
    use std::convert::From;
    #[derive(Debug)]
    struct Number {
        value: i32,
    }
    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item}
        }
    }
    let num = Number::from(30);
    println!("My number is {:?}", num);

    // Into trait就是把 From trait 倒过来而已
    // 也就是说，你为你的类型实现了 From，那么同时你也就免费获得了 Into
    // 使用 Into trait 通常要求指明要转换到的类型
    // 因为编译器大多数时候不断推断它
    // 我们现在可以直接使用 Into
    let int = 5;
    let num: Number = int.into();
    println!("My number is {:?}", num);
    
    // 6.2.TryFrom和TryInto
    println!("\n\n=====6.2.TryFrom和TryInto=====");
    // 类似于 From 和 Into，TryFrom和TryInto是类型转换的通用trait
    // 不同于 From/Into 的是，TryFrom 和 TryInto trait用于易出错的
    // 转换，也正因为如此，其返回值是 Result 型。
    // use std::convert::TryFrom;
    // use std::convert::TryInto;
    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);
    impl TryFrom<i32> for EvenNumber {
        type Error = ();
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }
    // TryFrom
    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));
    
    // TryInto
    let result: Result<EvenNumber, ()> = 8_i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5_i32.try_into();
    assert_eq!(result, Err(()));

    // 6.3.ToString和FromStr
    println!("\n\n=====6.3.ToString和FromStr=====");
    // 要把任何类型转换成为 String ,只需要实现那个类型的 ToString trait。然而
    // 不要直接这么做，您应该实现 fmt::Display trait，它会自动提供 ToString，
    // 并且还可以用来打印类型，就像 print! 那样。
    use std::fmt;
    struct Circle {
        redius: i32
    }
    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.redius)
        }
    }
    let circle = Circle{ redius: 6 };
    println!("Circle to string is: {}", circle.to_string());

    // // 另外一个实现 ToString 的例子
    // // use std::string::ToString;
    // impl ToString for Circle {
    //     fn to_string(&self) -> String {
    //         format!("圆的半径是：{:?}", self.redius)
    //     }
    // }
    // println!("{}", circle.to_string());
    // 编译出错！！！ 2023年1月28日19时43分24秒

    // 解析字符串
    // 经常需要把字符串转成数字，通过用 parse 函数
    // 通常得提供要转换到的类型，这可以通过不使用类型推断，或者用“涡轮鱼”语法;(turbo fish, <>)实现
    // 只要对目标类型实现了 FromStr trait，就可以用 parse 把字符串转换成目标类型，
    // 标准库实现了无数种类型的 FromStr。如果要转换到用户自定义类型，只要手动实现 FromStr 即可
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("the Sum: {:?}", sum);

    // 7.表达式
    println!("\n\n=====7.表达式=====");
    // Rust 程序（大部分）由一系列语句构成：
    // Rust 有多种语句，最普通的语句类型有两种：
    //      一种是声明绑定变量
    //      一种是表达式带上英文分号
    // 代码块也是表达式，所以它们可以用途赋值中的值
    //      代码块中的最后一个表达式将赋给适当的表达式，例如：局部变量
    //      但是，如果代码块的最后一个表达式结尾处有分号，则返回值为（）
    let x = 5_u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;
        // 将下列表达式赋给了 `y`
        x_cube + x_squared + x
    };
    let z = {
        // 分号结束了这个表达式，于是将 `()` 赋给了 `z`
        2 * x;
    };
    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);

    // 8.流程控制
    println!("\n\n=====8.流程控制=====");
    // 任何编程语言都包含一个必要部分就是改变控制流程，比如 if/else, for等

    // 8.1.if/else
    println!("\n\n=====8.1.if-else=====");
    // if-else 分支判断与其它语言不同的是，Rust语言中的布尔判断条件 不必 使用 小括号 包裹
    // 且每个条件后面 都 跟着一个代码块
    // if-else 条件选择是一个 表达式，并且所有分支 都 必须 返回相同的类型。
    let n = 5;
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }
    let big_n = 
        if n < 10 && n > -10 {
            println!(", and is a small number, inrease ten-fold");
            // 这个表达式返回一个 `i32` 类型
            10 * n
        } else {
            println!(", and is a big number, half the number");
            // 这个表达式也必须返回一个 `i32` 类型
            n / 2
        };
    println!("{} -> {}", n, big_n);
    
    // 8.2.loop循环
    println!("\n\n=====8.2.loop循环=====");
    // 可以使用 break 语句在任何时候退出一个循环
    // 也可以使用 continue 跳过循环体的剩余部分并开始下一轮循环
    let mut count = 0_u32;
    println!("Let's count until infinity!");
    // 无限循环
    loop {
        count += 1;
        if count == 3 {
            println!("three");
            continue;
        }
        println!("计数：{}", count);
        if count == 5 {
            println!("Ok, that's enough!");
            break;
        }
    }

    // 8.2.1.嵌套循环和标签
    println!("\n\n=====8.2.1.嵌套循环和标签=====");
    // 处理嵌套循环的时候可以 break 或 continue 外层循环
    // 在这种情况下，循环必须用 `label（标签）来注明，并且标签必须传递给 break/continue 语句
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the inner loop");
            // 这只是中断内部的循环
            // break;
            // 这会中断外部的循环
            break 'outer;
        }
        println!("This point will never be reached");
    }
    println!("Exited the oute loop");
    
    // 8.2.2.从loop循环中返回
    println!("\n\n=====8.2.2.从loop循环中返回=====");
    // loop有个用途是尝试一个操作直到成功为上
    //      若操作返回一个值，则 可能需要将其传递给代码的其余部分
    //      将该放在 break 之后，它就会被 loop 表达式返回
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // 这种情况还是我第一次见到，这个好玩。哈哈，2023年1月28日21时15分2秒
            break counter * 2;
        }
    };
    assert_eq!(result, 20);

    // 8.3.while循环
    println!("\n\n=====8.3.while循环=====");
    // 用 while 写一下臭名昭著的 FizzBuzz ，https://leetcode-cn.com/problems/fizz-buzz/
    // 下面是这个问题的描述：
    // 给你一个整数 n ，找出从 1 到 n 各个整数的 Fizz Buzz 表示，并用字符串数组 answer（下标从 1 开始）返回结果，其中：
    //     answer[i] == "FizzBuzz" 如果 i 同时是 3 和 5 的倍数。
    //     answer[i] == "Fizz" 如果 i 是 3 的倍数。
    //     answer[i] == "Buzz" 如果 i 是 5 的倍数。
    //     answer[i] == i （以字符串形式）如果上述条件全不满足。
    // 如果理解了题意，不难，但理解不了。2023年1月28日21时23分31秒
    let mut n = 1;
    let counter = 25;
    while n < counter {
        if n % 15 == 0 {
            print!("fizzbuzz");
        } else if n % 3 == 0 {
            print!("fizz");
        } else if n % 5 == 0 {
            print!("buzz");
        } else {
            print!("{}", n);
        }
        n += 1;
        if n < counter {
            print!(",");
        } else {
            println!();
        }
    }
    
    // 8.4.for循环
    println!("\n\n=====8.4.for循环=====");
    // for 与 区间
    // for in 结构可以遍历一个 Iterator（迭代器）
    //      创建迭代器的一个最简单方法是使用区间标记 a..b ，
    //      这会生成一个从 a（包含此值） 到  b（不含此值）的，步长为1的一系列值
    // 重写一下 FizzBuzz 程序
    let counter = 25;
    for n in 1..counter {
        if n % 15 == 0 {
            print!("fizzbuzz");
        } else if n % 3 == 0 {
            print!("fizz");
        } else if n % 5 == 0 {
            print!("buzz");
        } else {
            print!("{}", n);
        }
        // 这里要注意一下，终止的条件如何 ， 2023年1月28日21时39分5秒
        if n < counter - 1 {
            print!(",");
        } else {
            println!();
        }
    }
   
    // for与迭代器
    // for in 结构能以几种方式与 Iterator 互动，
    // 如果没有特殊指定，for 循环会对给出的集合应用 into_iter 函数，把它转换成一个迭代器
    // 转换成迭代器的方法还有 iter 和 iter_mut 函数
    // 这三个函数会以不同的 方式 返回集合中的数据
    //      iter - 在每次迭代中借用集合中的一个元素，这样集合本身不会被改变，循环之后仍可以使用
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    println!("iter() 其内部的内容不会被改变，names: {:?}\n", names);
    
    //      into_iter - 会消耗集合。在每次迭代中，集中的数据本身会被提供，一旦集合被消耗了，
    //                  之后无法再使用了，因为它已经在循环中被 ”移除”(move) 了。
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // 这里编译会出错，因为已经被 move 了！！2023年1月28日23时1分51秒
    println!("into_iter()将无法再访问数组内容，因为被移除了！！\n");
    
    //      iter_mut - 可变地(mutably) 借用集合中的每个元素，从而允许集合被就地修改
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => {
                "Hello"
            },
        }
    }
    // 我想把数组的内容更改为 “Hello {name}”,怎么实现？ 2023年1月28日23时14分20秒
    println!("iter_mut() 其内容可以被改变，names: {:?}", names);

    // 8.5.match匹配
    println!("\n\n=====8.5.match匹配=====");
    // Rust 通过 match 关键字提供模式匹配
    // 第一个匹配分支会被比对，并且所有可能的值都必须被覆盖
    let number = 123;
    println!("Tell me about {}", number);
    match number {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 | 23 => println!("This is a prime"),
        13..=19 => println!("A teen"),
        _ => println!("Ain't special"),
    }
    let boolean = true;
    let binary = match boolean {
        // match 必须覆盖所有可能的值
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);

    // 8.5.1.解构
    println!("\n\n=====8.5.1.解构=====");

    // 8.5.1.1.解构元组
    println!("\n\n=====8.5.1.1.解构元组=====");
    let triple = (3, -2, 3);
    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is '0', 'y' is {:?}, and 'z' is {:?}", y, z),
        (1, ..) => println!("First is '1' and the rest doesn't matter"),        // .. 用来表示元组的其他部分
        _ => println!("It doesn't matter what they are"),
    }
    
    // 8.5.1.2.解构枚举
    println!("\n\n=====8.5.1.2.解构枚举=====");
    #[allow(dead_code)]
    enum Color {
        Red,
        Blue,
        Green,
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red => println!("红色"),
        Color::Blue => println!("蓝色"),
        Color::Green => println!("绿色"),
        Color::RGB(r, g, b) => println!("Red: {}, green: {}, Blue: {}", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) => println!("Cyan: {}, magenta: {}, yellow: {}, key(black): {}", c, m, y, k),

    }
    
    // 8.5.1.3.指针和引用
    println!("\n\n=====8.5.1.3.指针和引用=====");
    // 对指针而言，解构(destructure) 与 解引用(dereference) 要区分开
    // 解引用 *
    // 解构 & ref 及 ref mut

    // 获得一个 ‘i32’ 类型的引用， & 表示解引用
    let reference = &4;
    match reference {
        // 如果用 '&val' 这个模式去匹配 'reference'，相当于做这样的比较：
        // '&i32'，即 'reference' 的类型
        // '&val'，即用于匹配的模式
        //  ^ 如果去掉匹配的 '&' ，'i32'应该赋给 'val'
        // 因此可用 'val' 表示被 'refrence' 引用的值 4
        &val => println!("Got a value via destructuring: {:?}", val),
    }
    // 如果不想用'&'，需要在匹配前解引用
    match *reference {
        val => println!("Got a value via dereference: {:?}",val),
    }
    
    // 如果一开始就不用引用，会怎样？
    // 'reference'是一个 '&' 类型，因为赋值语句的右边已经是一个引用，但下面这个不是引用，因为右边不是
    let _not_a_reference = 3;

    // Rust 对这种情况提供了 'ref'。它更改了赋值行为，从而可以对具体值创建引用。
    // 下面这行将得到一个引用 
    let ref _is_a_reference = 3;

    // 相应的，定义两个非引用的变量，通过 'ref' 和 'ref mut' 仍可取得其引用。
    let value = 5;
    let mut mut_value = 9;

    // 使用 'ref' 关键字来创建引用。
    // 下面的 r 是 'i32' 类型，它像 'i32' 一样可以直接打印，因此用法上似乎看不出什么区别。
    // 但可以把 'println!' 中的 'r' 改成 '*r' ，仍然能正常运行
    // 而前面的例子中的 'println!' 时就 不能 是 '*val'，因为不能对整数进行解引用
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    } 
    // 类似地使用 'ref mut'。
    match mut_value {
        ref mut m => {
            // 已经获得 'mut_value' 的引用，先要解引用，才能改变它的值。
            * m += 10;
            println!("We add 10. 'mut_value' is: {:?}", m);
        },
    }
    
    //region 8.5.1.4.结构体
    println!("\n\n=====8.5.1.4.结构体=====");
    // 解构 struct
    struct Foo { x: (u32, u32), y: u32 }
    // 解构结构体的成员
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;
    println!("a = {}, b = {}, y = {}", a, b, y);
    // 解构结构体并重命名变量，成员顺序并不重要 
    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);
    // 也可以忽略某些变量
    let Foo { y, .. } = foo;
    println!("y = {}, 其它变量忽略了。", y);

    // 8.5.2.卫语句
    println!("\n\n=====8.5.2.卫语句=====");
    // '卫'即保卫的意思，guard
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation.."),
    }
    //endregion
    
    //region 8.5.3.绑定
    println!("\n\n=====8.5.3.绑定=====");
    // 在 match 中，若间接地访问一个变量，则 不经过重新绑定就无法在分支中再使用它
    // match 提供了 @ 符号来绑定变量到名称
    fn age() -> u32 {
        15
    }
    println!("Tell me what type of person you are");
    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        n @ 1 ..=12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..=19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm anold person of age {:?}", n),
    }
    // 也可以使用绑定来解构 enum 变体
    fn some_number() -> Option<u32> {
        Some(11)
    }
    match some_number() {
        // 获取 'Some' 可变类型，如果它的值(绑定到'n'上面)等于 42 ，则 匹配
        Some(n @ 42) => println!("The Answer is: {}", n),
        Some(n) => println!("Not interesting...{}", n),
        _ => println!("缺少值？？？")
    }
    //endregion

    //region 8.6.if let
    println!("\n\n=====8.6.if let=====");
    // match 用到赋值语句中去
    let optional = Some(7);
    match optional {
        // 这里是从 optional 中解构出 'i'
        Some(i) => println!("This is a really long string and '{:?}'", i),
        // 这行是必须要有的，因为 'match' 需要覆盖全部情况
        _ => {}, 
    }
    // 如果用 if let 处理上面的情况，则会简洁得多，并且允许指明数种失败情况下的选项
    // 全部都是 'Option<i32>' 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // if let 结构读作： 若 'let' 将 'number' 解构成 'Some(i)' 则执行语句块（'{}'）
    if let Some(i) = number {
        println!("Matched {:?}", i);
    };
    // 如果要指明失败情况，就使用 else
    if let Some(i) = letter {
        println!("Matched {:?}", i);
    } else {
        // 解构失败
        println!("Didn't match a number. Let's go with a letter!");
    };
    // 提供另一种失败情况下的条件
    let i_like_letters = false;
    if let Some(i) = emoticon {
        println!("Matched {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 条件的值为 false ，于是以下是默认的分支
        println!("I don't like letters. Let's go with an emoticon :)!");
    };

    // 同样，可以用if let 匹配任何枚举值
    enum MyFoo {
        Bar,
        Baz,
        Qux(u32)
    }
   
    let a = MyFoo::Bar;
    let b = MyFoo::Baz;
    let c = MyFoo::Qux(100);
    if let MyFoo::Bar = a {
        println!("a is foobar");
    }
    if let MyFoo::Bar = b {
        println!("b is foobar");
    }
    if let MyFoo::Qux(value) = c {
        println!("c is {}", value);
    }
    // 另一个好处是，if let 允许匹配枚举非参数化的变量，即枚举未注明 #[derive(PartialEq)]，也
    // 没有为其实现 PartialEq。在这种情况下，通常  if Foo::Bar==a 会出错，因为此类枚举的实例
    // 不具有可比性，而 if let 是可行的
    //endregion

    //region 8.7.while let
    println!("\n\n=====8.7.while let=====");
    // 和 if let 类似，while let 也可以把别扭的 match 改写得好看一些
    // 将 'optional' 设为 'Option<i32>' 类型
    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {
                if i > 9 {
                    println!("Greater than 9, quit!");
                    optional = None;
                } else {
                    println!("'i' is '{:?}'. Try again.", i);
                    optional = Some(i + 1);
                }
            },
            _ => { break; }
        }
    }
    // 下面用 while let 改写
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("'i' is '{:?}'. Try again!", i);
            optional = Some(i + 1);
        }
       // 使用的缩进更少，并且不用显式处理失败情况
    }
    //endregion

    //region 9.函数
    println!("\n\n=====9.函数=====");
    // 又重写 FizzBuzz，呵呵
    fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
        if rhs == 0 {
            return false;
        }
        lhs % rhs ==0
    }
    fn fizzbuzz(n: u32)  -> () {
        if is_divisible_by(n, 15) {
            print!("fizzbuzz");
        } else if is_divisible_by(n, 3) {
            print!("fizz");
        } else if is_divisible_by(n, 5) {
            print!("buzz");
        } else {
            print!("{}", n);
        }
    }
    fn fizzbuzz_to(n: u32) {
        for i in 1..n {
            fizzbuzz(i);
            if i < n - 1{
                print!(",");
            } else {
                println!();
            }
        }
    }
    fizzbuzz_to(25);
    //endregion
    
    //region 9.1.方法
    println!("\n\n=====9.1.方法=====");
    // 方法(method)是依附于对象的函数，这些方法通过关键字 self 来访问对象中的数据和其他
    // 方法在 impl 代码块中定义
    struct Point {
        x: f64,
        y: f64,
    }
    // 实现的代码码，'Point' 的所有方法都在这里给出
    impl Point {
        // 静态方法，static method，不需要被实例调用，一般用作构造器(constructor)
        fn origin() -> Point {
            Point { x:0.0, y:0.0 }
        }
        // 另一个静态方法
        fn new(x: f64, y: f64) -> Point {
            Point { x:x, y:y }
        }
    }
    struct Rectangle {
        p1: Point,
        p2: Point,
    }
    impl Rectangle {
        // 实例方法（instance method），
        // '&self' 是 'self:&Self' 的语法粮(sugar) 
        // 其中'Self'是方法调用者的类型
        fn area(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            ((x1 - x2) * (y1 - y2)).abs()
        }
        fn perimeter(&self) -> f64 {
            let Point { x: x1, y: y1 } = self.p1;
            let Point { x: x2, y: y2 } = self.p2;
            2.0 * ((x1 - x2).abs() * (y1 - y2).abs())
        }
        // 这个方法要求调用者是可变的
        // '&mut self' 为 ;'self: &mut Self' 的语法糖
        fn translate(&mut self, x: f64, y: f64) {
            self.p1.x += x;
            self.p2.x += x;
            self.p1.y += y;
            self.p2.y += y;
        }
    }
    // 'Pair' 拥有资源：两个堆分配的整形
    struct Pair(Box<i32>, Box<i32>);

    impl Pair {
        // 此方法会'消耗'调用者的资源
        // 'self' 为 'self: Self' 的语法糖
        fn destroy(self) {
            // 解构 'self'
            let Pair(first, second) = self;
            println!("Destorying Pair({}, {})", first, second);
            // 'first' 和 'second' 离开作用域后释放
        }
    }
   
    let rectangle = Rectangle {
        // 注意：静态方法要使用双冒号调用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    // 实例方法通过点运算符来调用 
    // 第一个参数 '&self' 是隐式传递的，亦即：
    // 'rectangle.perimeter()' == 'Rectangle::perimeter(&rectangle)'
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    // rectangle.translate(1.0, 1.1);
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();
    //endregion

    //region 9.2.闭包
    println!("\n\n=====9.2.闭包=====");
    // Rust 中的闭包(closure)，也称之为lambda表达式或者 lambda，是一类
    // 能够捕获周围作用域中变量的函数。例如，一个可以捕获 x 变量的闭包如下：
    //          |val| val + x
    // 其语法和能力使其在临时(on the fly)使用相当方便。
    // 调用一个闭包和调用一个函数完全相同，不过调用闭包时，
    // 输入和返回类型两者都可以自动推导，而输入变量名必须指明
    // 其他的特点包括：
    //      声明时使用 || 替代 () 将输入参数括起来。
    //      函数体定界符 ({}) 对于单个表达式是可选的，其他情况必须加上
    //      有能力捕获外部环境的变量
    // 
    // 通过闭包和函数分别实现自增，这个是函数实现
    fn function(i: i32) -> i32 { i + 1 }
    // 闭包匿名的，这里我们将其绑定到引用。
    // 类型标注和函数的一样，不过类型标准和使用 '{}' 来围住函数体是可选的
    // 这些匿名函数(nameless function)被赋值赋值给合适的命名的变量
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;
    // 闭包表达式产生的类型就是 闭包类型，不属于引用类型，
    // 确实无法对上面的两个变量进行 解引用。
    let i = 1;
    // 调用函数和闭包
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    // 没有参数的闭包，返回一个 'i32' 类型
    // 返回类型是自动推导的
    let one = || 1;
    println!("closure returning one: {}", one());
    //endregion

    //region 9.2.1.捕获
    // 9.2.1.捕获
    println!("\n\n=====9.2.1.捕获=====");
    // 闭包本质上很灵活，能做功能要求的事情，使闭包在没有类型标注的情况下运行。
    // 这使得捕获 (capture) 能够灵活地适应用例，既可移动 (move) ，又可借用 (borrow)
    // 闭包可以通过以下方式捕获变量：
    //      通过引用： &T
    //      通过可变引用： &mut T
    //      通过值： T
    // 闭包优先通过引用来捕获变量，并且仅在需要时使用其他方式！！！
    use std::mem;
    let color = String::from("green");
    // 下面的闭包会打印 'color'，它会立即借用(通过引用，'&')'color'并将该借用和闭包本身
    // 存储到 'print' 变量中。
    // color 会一直保持被借用状态直到 'print' 离开作用域。
    let print = || println!("color is: {}", color);
    // 使用借用来调用闭包 'color'
    print();
    // 'color' 可再次被不可变借用，因为闭包只持有一个指向 'color' 的不可变引用。
    let _reborrow = &color;
    print();

    // 在最后使用 'print' 之后，移动或重新借用都是允许的
    let _color_moved = color;

    let mut count = 0;
    // 这个闭包使 'count' 值增加，要做到这点，它需要得到 '&mut count' 或者 'count' 本身，
    // 但 '&mut count' 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 'count'
    // 'inc' 前面需要加上'mut'，因为闭包里存储着一个 '&mut' 变量，调用闭包时，该变量的变化
    // 就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("'count' is: {}", count);
    };
    // 使用可变借用调用闭包
    inc();

    // 因为之后调用闭包，所以仍然可变借用 'count'。试图重新借用将导致错误
    // let _reborrow = &count;
    inc();

    // 闭包不再借用 '&mut count' ，因此可以正确地重新借用
    let _count_reborrowed = &mut count;

    // 不可复制类型 (non-copy type)
    let movable = Box::new(3);
    // 'mem::drop' 要求 'T' 类型本身，所以闭包将会捕获变量的值，这种情况下，
    // 可复制类型将会提制给闭包，从而原始值不受影响。
    // 不可复制类型必须移动 (move) 到闭包中，因而 'movable' 变量在这里立即
    // 移动到了闭包中。
    let consume = || {
        println!("'movable': {:?}", movable);
        mem::drop(movable);
    };
    // 'consume' 消耗了该变量，所以该闭包只能调用一次。
    consume();
    // 在竖线 | 之前使用 move 会强制闭包取得被捕获变量的所有权。
    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    //endregion

    //region 9.2.2.作为输入参数
    // 9.2.2.作为输入参数
    println!("\n\n=====9.2.2.作为输入参数=====");
    // 虽然 Rust 无需类型说明就能在大多数时候完成变量捕获，但在写函数时，这种模糊写法是不的。
    // 当以闭包作为输入参数时，必须指出闭包的完整类型，它是通过使用以下 trait 中的一种来指定的。
    // 其受限程序按以下顺序递减：
    // Fn : 表示捕获方式为通过引用(&T)的闭包
    // FnMut : 表示捕获方式为通过可变引用 (&mut T) 的闭包
    // FnOnce ： 表示捕获的方式为通过值 (T) 的闭包
    // 顺序之所以是这样的，是因为 &T 只是获取了不可变的引用，&mut T 则可以改变变量，T 则是拿到了
    // 变量的所有权而非借用。
    // 对闭包所要捕获的每个变量，编译器都将以限制最少的方式来捕获。
    // 例如用一个类型说明为 FnOnce 的闭包作为参数。
    // 这说明闭包可能采取 &T，&mut T 或 T 捕获变量。

    // 该函数将闭包作为参数并调用它
    fn apply<F>(f: F) where
        F: FnOnce() {
        f();
    }
    // 输入闭包，返回一个 'i32' 整形的函数。
    fn apply_to_3<F>(f: F) -> i32 where
        F: Fn(i32) -> i32 {
        f(3)
    }
    let greeting = "hello";

    // 不可复制类型，'to_owned' 从的数据创建 有所有权的数据。
    let mut farewell = "goodbye".to_owned();

    // 捕获两个变量：通过引用捕获 'greeting'，通过值捕获 'farewell'。
    let diary= || {
        // 'greeting' 通过引用捕获，因为要求闭包通过可变引用来捕获它。
        println!("I said {}.", greeting);

        // 下文改变了 'farewell'， 因而要求闭包通过可变引用来捕获它。
        // 现在需要 'FnMut'。
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzz");
        mem::drop(farewell);
    };
    // 以闭包作为参数，调用函数 'apply'。
    apply(diary);

    // 闭包 'double' 满足 'apply_to_3' 的 trait 约束。
    let double = |x: i32| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
    //endregion9.2.2.作为输入参数

    //region 9.2.3.类型匿名
    println!("\n\n=====9.2.3.类型匿名=====");
    // 闭包从周围的作用域中捕获变量是简单明了。
    // 观察一下使用闭包作为函数参数，这要求闭包是泛型的，定义的方式决定了这是必要的。
    // 当闭包被定义，编译器会隐式地创建一个匿名类型的结构体，用以储存捕获的变量，同时
    // 为这个未知类型的结构体实现函数功能，通过 Fn/FnMut/FnOnce 三种 trait 中的一种
    // 若使用闭包作为函数参数，由于这个棱柱体的类型未知，任何的用法都要求是泛型的
    // 然而，使用未限定类型的参数 <T> 过于不明确，并且是不允许的。
    // 事实上，指明为该结构体实现的是 Fn/FnMut/FnOnce 中的哪种 trait，对于
    // 约束该结构体的类型而言已经足够了。
    // 'F' 必须 为一个没有输入参数和返回值的闭包实现 'Fn'
    // 这和对 'print' 的要求恰好是一样的。 2023年1月30日21时4分32秒
    fn my_apply<F>(f: F) where
        F: Fn() {
        f();
    }
    let x = 7;
    // 捕获 'x' 到匿名类中，并为它实现 'Fn'。将闭包存储到 'print' 中去。
    let print = || println!("{}", x);
    my_apply(print);
    //endregion

    //region 9.2.4.输入函数
    println!("\n\n=====9.2.4.输入函数=====");
    // 既然闭包可以作为参数，很可能想知道函数是否也可以呢？
    // 确实可以！！！
    // 如果声明一个接受闭包作为参数的函数，那么任何满足该闭包的 trait 约束的函数都可以作为其参数

    // 定义一个函数，可以接受一个由 'Fn' 限定的泛型 'F' 参数并调用它。
    fn call_me<F: Fn()>(f: F) {
        f()
    }
    // 定义一个满足 'Fn' 约束的封闭函数 (wrapper function)。
    fn my_function() {
        println!("I'm a function");
    }

    // 定义一个满足 'Fn' 约束的闭包。
    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(my_function);
    //endregion

    //region 9.2.5.作为输入参数
    println!("\n\n=====9.2.5.作为输入参数=====");
    // 闭包作为输入参数是可能的，所以返回闭包作为输出参数 (output parameter) 也是可能的。
    // 然而返回闭包类型会有问题，因为目前 Rust 只支持返回具体(非泛型)的类型。
    // 按照定义，匿名的闭包类型是未知的，所以只有使用 impl Trait 才能返回一个闭包。
    // 返回闭包的有效特征是： Fn/FnMut/FnOnce
    // 除此之外，还必须使用 move 关键字，它表明所有的捕获都是通过 值 进行的。这是必须的
    // 因为在函数退出时，任何通过引用的捕获都会被丢弃，会在闭包中留下无效的引用。
    fn create_fn() -> impl Fn() {
        let text = "Fn".to_owned();
        move || println!("This is a : {}", text)
    }

    fn create_fnmut() -> impl FnMut() {
        let text = "FnMut".to_owned();
        move || println!("This is a : {}", text)
    }
    fn create_fnonce() -> impl FnOnce() {
        let text = "FnOnce".to_owned();
        move || println!("This is a : {}", text)
    }

    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    fn_plain();
    fn_mut();
    fn_once();
    //endregion

    //region 9.2.6.1.Iterator::any
    println!("\n\n=====9.2.6.1.Iterator::any=====");
    // Iterator::any 是一个函数，若传给它一个迭代器 (Iterator)，当其中任一元素满足谓词(predicate)
    // 时它将返回 true，否则返回 false。还是直接看例子吧。
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    // 对 vec 的 'iter()' 举出 '&i32'。通过用 '&x' 匹配，把它解构成 'i32'。
    // any 方法会自动地把 'vec.iter()' 枚举的迭代器的元素一个个地传给闭包。
    // 因此，闭包接收到的参数是 '&i32' 类型的。
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // 对 vec 的 'into_iter()' 枚举出 'i32' 类型，无需解构。
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));
    // 针对上面的写法，我们平时也只是会使用罢了，但原理，或者说为什么这样写，是一点也不清楚的
    // 而这个‘知其然而不知其所以然’，正是自己不求其解，从而得不到真正提高的主要原因
    // 2023年1月30日21时59分40秒

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];
    // 对数组的 'iter()' 枚举出 '&i32'。
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // 对数组的 'into_iter()' 枚举出 'i32'。
    println!("2 in array2: {}", array2.into_iter().any(|x| x == 2));

    //endregion

    //region 9.2.6.2.Iterator::find
    println!("\n\n=====9.2.6.2.Iterator::find=====");
    // Iterator::find 是一个函数，在传给它一个迭代器时，将用 option 类型返回第一个满足库房的元素。
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    // 对迭代器举出的元素的引用是 '&&i32' 类型。解构成 'i32' 类型。
    // 注意：'find' 方法会把迭代器元素的引用传给闭包。迭代器元素自身是 '&i32' 类型，
    // 所以传给闭包的是 '&&i32' 类型。
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));
    //endregion

    //region 9.3.高阶函数
    println!("\n\n=====9.3.高阶函数=====");
    // Rust 提供了高阶函数 (Higher Order Function,HOF)，指那些输入一个或多个函数，
    // 并且/或者产生一个更有用的函数的函数。
    // HOF和惰性(lazy iterator)给Rust带来了函数式(Functional)编程的风格。
    fn is_odd(n: u32) -> bool {
        n % 2 == 1
    }
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;
    // 命令式(imperative)写法
    let mut acc = 0;
    for n in 0.. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imperative style: {}", acc);

    // 函数式的写法
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)
            .take_while(|&n| n < upper)
            .filter(|&n| is_odd(n))
            .fold(0, |sum, i| sum + i);
    println!("functional style: {}", sum_of_squared_odd_numbers);
    //endregion

    //region 9.4.发散函数
    println!("\n\n=====9.4.发散函数=====");
    // 发散函数(diverging function)绝不会返回，使用 ！ 标记，这是一个空类型。
    fn some_fn() {
        ()
    }

    let a: () = some_fn();
    println!("This function returns and you can see this line.");
    // 这种类型的主要优点是它可以被转换成为任何其它类型，从而可以在需要精确凿开的地方使用
    // 例如在 match 匹配分配，例：
    fn sum_odd_numbers(up_to: u32) -> u32 {
        let mut acc = 0;
        for i in 0..up_to {
            let addition: u32 = match i % 2 == 1 {
                // i 变量的类型为 u32，这毫无问题
                true => i,
                false => continue,
            };
            acc += addition;
        }
        acc
    }
    println!("Sum of odd numbers up to 9(excluding): {}", sum_odd_numbers(9));
    //endregion
}
