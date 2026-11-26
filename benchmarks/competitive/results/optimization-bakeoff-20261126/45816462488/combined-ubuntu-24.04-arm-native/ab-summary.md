| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.385x |
| materialize | 0.998x | 0.994x | 1.248x |
| perft_bulk_d4 | 1.000x | 0.999x | 1.239x |
| perft_full_d3 | 1.001x | 0.996x | 1.073x |
| fen_parse | 1.001x | 0.998x | 1.253x |
| fen_write_native_ep | 1.000x | 0.997x | 1.577x |
| snapshot | 0.988x | 0.983x | 0.948x |
| san_parse_all | 1.001x | 0.996x | 1.808x |
| san_render_native | 0.998x | 0.995x | 2.518x |
| apply_children_full | 1.002x | 0.998x | 1.180x |
| make_unmake_full | 0.999x | 0.991x | 1.479x |
| startpos | 1.000x | 1.000x | 1.304x |
| chess960_materialize | 1.004x | 1.002x | 1.399x |
| chess960_perft_d3 | 0.998x | 0.997x | 1.338x |
| polyglot_key | 1.008x | 0.992x | 25.394x |
| polyglot_recompute | 1.000x | 0.999x | 1.073x |
| repetition_key | 1.003x | 1.002x | 18.352x |
| corpus_count | 1.001x | 0.996x | 1.232x |
| corpus_materialize | 0.998x | 0.997x | 1.164x |
| corpus_uci_sorted | 0.998x | 0.994x | 1.446x |
| replay_trusted_native | 0.993x | 0.986x | 0.538x |
| replay_checked_native | 1.008x | 1.008x | 1.033x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.99x).
