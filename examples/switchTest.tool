int32 a = 3

func void matchCase(int32 a)
	switch a % 4:
	case 0:
		println("Integer a is of the 0 remainder class, modulus 4.")
	case 1:
		println("Integer a is of the 1 remainder class, modulus 4.")
	case 2:
		println("Integer a is of the 2 remainder class, modulus 4.")
	case 3:
		println("Integer a is of the 3 remainder class, modulus 4.")
	end
end