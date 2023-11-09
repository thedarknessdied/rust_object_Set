​	当我使用println!&io::stdin().read_line()进行输入输出组合的时候，一切都看得是那么的正常。

```rust
	println!("Please enter a random number between 1 and 100:");
	io::stdin() 
		.read_line(&mut guess_number)
        	.expect("Anything wrong happened!");
```

![normal](https://github.com/thedarknessdied/rust_object_Set/blob/main/problem/Print!_And_println!_Handling_Differences_in_Buffers/README.assets/normal.png)

​	但是当我使用print!&io::stdin().read_line()进行输入输出组合的时候，一切的输入输出就是那么的不尽人意，输入和输出语句都发生了颠倒错位。

```rust
	print!("Please enter a random number between 1 and 100:");
	io::stdin() 
		.read_line(&mut guess_number)
        	.expect("Anything wrong happened!");
```

![wrong](https://github.com/thedarknessdied/rust_object_Set/blob/main/problem/Print!_And_println!_Handling_Differences_in_Buffers/README.assets/wrong.png)

​	显而易见的是，他们都存在C/C++对存、取缓冲区数据异常的问题，那么我该如何解决这个问题？
