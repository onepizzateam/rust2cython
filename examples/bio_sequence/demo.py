import sys
sys.path.insert(0, '.')
import bio_sequence as bio

seq = "ATGCGCATTAGC"

print(f"sequence:           {seq}")
print(f"GC content:         {bio.gc_content(seq):.1f}%")         # 41.7%
print(f"count G:            {bio.count_nucleotide(seq, 'G')}")   # 2
print(f"reverse complement: {bio.reverse_complement(seq)}")
print(f"contains ATG:       {bio.contains_motif(seq, 'ATG')}")   # True
print(f"contains TTT:       {bio.contains_motif(seq, 'TTT')}")   # False
