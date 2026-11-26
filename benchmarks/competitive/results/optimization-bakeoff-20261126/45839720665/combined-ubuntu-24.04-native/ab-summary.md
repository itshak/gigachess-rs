| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.995x | 1.479x |
| materialize | 1.097x | 1.047x | 1.614x |
| perft_bulk_d4 | 1.003x | 0.997x | 1.171x |
| perft_full_d3 | 1.004x | 1.001x | 1.271x |
| fen_parse | 0.941x | 0.824x | 1.063x |
| fen_write_native_ep | 1.000x | 0.977x | 1.923x |
| snapshot | 1.006x | 1.001x | 1.154x |
| san_parse_all | 1.031x | 0.999x | 1.940x |
| san_render_native | 1.017x | 1.011x | 2.110x |
| apply_children_full | 1.028x | 1.018x | 1.333x |
| make_unmake_full | 0.986x | 0.936x | 1.162x |
| startpos | 1.117x | 1.117x | 1.119x |
| chess960_materialize | 0.991x | 0.972x | 1.452x |
| chess960_perft_d3 | 1.017x | 1.014x | 1.518x |
| polyglot_key | 0.969x | 0.935x | 38.270x |
| polyglot_recompute | 1.033x | 1.028x | 1.000x |
| repetition_key | 0.990x | 0.969x | 23.815x |
| corpus_count | 0.999x | 0.997x | 1.074x |
| corpus_materialize | 1.004x | 1.002x | 1.072x |
| corpus_uci_sorted | 1.016x | 1.008x | 1.566x |
| replay_trusted_native | 1.069x | 1.020x | 0.924x |
| replay_checked_native | 1.118x | 1.081x | 1.244x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.15x).
