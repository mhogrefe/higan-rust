pub fn normalize(mut color: u64, mut source_depth: u32, target_depth: u32) -> u64 {
    if source_depth == 0 || target_depth == 0 {
        return 0;
    }
    while source_depth < target_depth {
        color = (color << source_depth) | color;
        source_depth += source_depth;
    }
    if target_depth < source_depth {
        color >>= source_depth - target_depth;
    }
    color
}
