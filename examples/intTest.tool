int32 Int1 = 40
int32 Int2 = 5

func int32 sum(int32 a, int32 b)
	return a + b
end

int32 Int3 = sum(Int1, Int2) ## 45

func int32 diff(int32 a, int32 b)
	return a - b
end

int32 Int4 = diff(Int1, Int2) ## 35