start
b tauri_utils::assets::{impl#4}::get::{closure#0}
set $i = 0
while($i<100)
continue
set $base_address = *($rsi as *const usize)
eval "dump binary memory dump/%p.bin $base_address $base_address+*(($rsi+8) as *const usize)", $base_address
set $i = $i + 1
end
