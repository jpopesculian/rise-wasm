@external("env", "epoch_time")
export declare function epochTime(
    year: u32,
    month: u32,
    day: u32,
    hours: u32,
    minutes: u32,
    seconds: u32
): i64;

@external("env", "now")
export declare const NOW: i64;
