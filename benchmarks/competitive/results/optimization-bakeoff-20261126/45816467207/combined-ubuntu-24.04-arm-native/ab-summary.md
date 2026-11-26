| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.390x |
| materialize | 0.999x | 0.995x | 1.245x |
| perft_bulk_d4 | 0.996x | 0.990x | 1.244x |
| perft_full_d3 | 0.991x | 0.983x | 1.065x |
| fen_parse | 0.989x | 0.982x | 1.230x |
| fen_write_native_ep | 1.004x | 1.002x | 1.628x |
| snapshot | 1.013x | 1.008x | 0.949x |
| san_parse_all | 1.006x | 1.002x | 1.821x |
| san_render_native | 1.001x | 0.996x | 2.515x |
| apply_children_full | 0.997x | 0.990x | 1.178x |
| make_unmake_full | 0.990x | 0.979x | 1.464x |
| startpos | 0.999x | 0.999x | 1.302x |
| chess960_materialize | 0.997x | 0.996x | 1.394x |
| chess960_perft_d3 | 1.009x | 1.005x | 1.359x |
| polyglot_key | 1.040x | 1.005x | 25.989x |
| polyglot_recompute | 0.997x | 0.996x | 1.073x |
| repetition_key | 1.020x | 1.004x | 18.578x |
| corpus_count | 0.993x | 0.988x | 1.207x |
| corpus_materialize | 0.997x | 0.993x | 1.149x |
| corpus_uci_sorted | 0.997x | 0.991x | 1.447x |
| replay_trusted_native | 0.991x | 0.985x | 0.548x |
| replay_checked_native | 1.002x | 0.997x | 1.030x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.93x).
