/// Count occurrences of a nucleotide in a DNA sequence
pub fn count_nucleotide(sequence: &str, nucleotide: &str) -> i32 {
    let n = nucleotide.chars().next().unwrap_or('N');
    sequence.chars().filter(|&c| c == n).count() as i32
}

/// GC content as a percentage (0.0 to 100.0)
pub fn gc_content(sequence: &str) -> f64 {
    let total = sequence.len();
    if total == 0 { return 0.0; }
    let gc = sequence.chars().filter(|&c| c == 'G' || c == 'C').count();
    (gc as f64 / total as f64) * 100.0
}

/// Reverse complement of a DNA sequence
pub fn reverse_complement(sequence: &str) -> String {
    sequence.chars().rev().map(|c| match c {
        'A' => 'T', 'T' => 'A', 'G' => 'C', 'C' => 'G', _ => 'N'
    }).collect()
}

/// Check if sequence contains a motif
pub fn contains_motif(sequence: &str, motif: &str) -> bool {
    sequence.contains(motif)
}
