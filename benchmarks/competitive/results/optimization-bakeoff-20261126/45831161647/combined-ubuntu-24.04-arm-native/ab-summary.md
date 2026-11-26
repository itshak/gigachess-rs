| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.380x |
| materialize | 1.000x | 0.995x | 1.256x |
| perft_bulk_d4 | 0.998x | 0.996x | 1.240x |
| perft_full_d3 | 0.995x | 0.986x | 1.045x |
| fen_parse | 0.997x | 0.990x | 1.215x |
| fen_write_native_ep | 1.000x | 0.997x | 1.569x |
| snapshot | 0.989x | 0.981x | 0.958x |
| san_parse_all | 0.999x | 0.996x | 1.827x |
| san_render_native | 1.004x | 0.981x | 2.556x |
| apply_children_full | 0.989x | 0.965x | 1.141x |
| make_unmake_full | 1.020x | 0.999x | 1.574x |
| startpos | 1.000x | 1.000x | 1.334x |
| chess960_materialize | 1.007x | 1.006x | 1.401x |
| chess960_perft_d3 | 1.001x | 0.997x | 1.349x |
| polyglot_key | 0.977x | 0.955x | 25.339x |
| polyglot_recompute | 0.997x | 0.995x | 1.085x |
| repetition_key | 0.982x | 0.962x | 19.140x |
| corpus_count | 0.992x | 0.989x | 1.203x |
| corpus_materialize | 0.993x | 0.993x | 1.165x |
| corpus_uci_sorted | 1.001x | 0.995x | 1.455x |
| replay_trusted_native | 0.995x | 0.993x | 0.600x |
| replay_checked_native | 0.988x | 0.985x | 1.039x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.10x).
