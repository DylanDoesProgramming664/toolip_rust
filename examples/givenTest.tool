int32 a = 3

func matchCase(int32 x)
	given x % 4 do
		when 0 => println("Integer {} is of the 0 remainder class, modulus 4.", x)
		when 1 => println("Integer {} is of the 1 remainder class, modulus 4.", x)
		when 2 => println("Integer {} is of the 2 remainder class, modulus 4.", x)
		when 3 => println("Integer {} is of the 3 remainder class, modulus 4.", x)
	end
end

matchCase(a)