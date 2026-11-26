| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.117x | 1.006x | 1.477x |
| materialize | 1.045x | 0.985x | 1.157x |
| perft_bulk_d4 | 1.097x | 0.994x | 1.323x |
| perft_full_d3 | 1.001x | 0.970x | 1.030x |
| fen_parse | 0.880x | 0.868x | 1.123x |
| fen_write_native_ep | 1.000x | 0.997x | 1.854x |
| snapshot | 1.013x | 0.999x | 1.251x |
| san_parse_all | 1.000x | 0.989x | 2.170x |
| san_render_native | 1.010x | 0.995x | 2.108x |
| apply_children_full | 0.995x | 0.981x | 1.272x |
| make_unmake_full | 0.997x | 0.970x | 1.210x |
| startpos | 1.000x | 1.000x | 1.199x |
| chess960_materialize | 1.058x | 1.039x | 1.560x |
| chess960_perft_d3 | 1.065x | 1.037x | 1.579x |
| polyglot_key | 1.008x | 0.999x | 20.434x |
| polyglot_recompute | 0.978x | 0.953x | 1.002x |
| repetition_key | 1.000x | 0.998x | 18.899x |
| corpus_count | 1.096x | 1.043x | 1.299x |
| corpus_materialize | 1.054x | 1.027x | 1.227x |
| corpus_uci_sorted | 1.002x | 1.001x | 1.652x |
| replay_trusted_native | 1.010x | 0.995x | 1.168x |
| replay_checked_native | 0.994x | 0.990x | 1.005x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.02x).
