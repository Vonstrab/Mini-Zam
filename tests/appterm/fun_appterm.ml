
let rec h x = if x < 10 then h (x-1) else x

let g x = h x

let f x = g x

let _ = f 234
