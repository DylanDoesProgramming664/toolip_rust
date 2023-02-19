int32 a = 3

func matchCase(int32 x)
	match x % 4:
		0 => println("Integer {} is of the 0 remainder class, modulus 4.", x)
		1 => println("Integer {} is of the 1 remainder class, modulus 4.", x)
		2 => println("Integer {} is of the 2 remainder class, modulus 4.", x)
		3 => println("Integer {} is of the 3 remainder class, modulus 4.", x)
	end
end

matchCase(a)