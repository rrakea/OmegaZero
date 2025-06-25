pub fn pritify_bitboard(bb: u64) {
    let mut buf = Vec::new();
    let bit_str = format!("{:064b}", bb); 
    for rank in 0..8 {
        let start = rank * 8;
        let end = start + 8;
        let row: String = bit_str[start..end].chars().rev().collect();
        buf.push(row);
    }

    println!("Bit: {:064b}", bb);
    println!("Board:\n{}\n", buf.join("\n"));
}
