-module(functions).
-compile(export_all). %% replace with -export() later, for God's sake!


old_enough(X) when X >= 16 -> true;
old_enough(_) -> false.

heh_fine() ->
if 1 =:= 1 ->
works
end,
if 1 =:= 2; 1 =:= 1 ->
works
end.
