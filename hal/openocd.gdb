target extended-remote :3333
break main

# cargo embed会自动load。
# openocd -f interface/stlink.cfg -f target/stm32f3x.cfg不会自动load，需要在gdb命令行中运行load。

# monitor reset halt 主动复位
