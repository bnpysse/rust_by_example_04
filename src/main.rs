#![allow(overflowing_literals)]
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
    


    
    
    
}
