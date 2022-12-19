let tz_multiply : int -> int -> int
  = fun _x _y -> _x * _y

let () =
  Callback.register "tz_multiply" tz_multiply