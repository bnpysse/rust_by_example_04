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
    // 当数据


    


    
    
    
}
