int32 Int1 = 1
int32 Int2 = 2

func int32 twoIntConcat(int32 a, int32 b)
	return a .. b
end

int32 Int3 = twoIntConcat(Int1, Int2) ## 12

int32 multiIntConcat;
multiIntConcat = func (int32 a, int32 etc)
	int32 sum
	for int32 i in 0 to #self.args do
		if i == 0 then
			sum = self.args[i]
		else
			sum ..= self.args[i]
		end
	end
	return sum
end