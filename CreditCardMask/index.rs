fn maskify(cc: &str) -> String {
    let mask_length = cc.len().saturating_sub(4);
    "#".repeat(mask_length) + &cc[mask_length..]
}