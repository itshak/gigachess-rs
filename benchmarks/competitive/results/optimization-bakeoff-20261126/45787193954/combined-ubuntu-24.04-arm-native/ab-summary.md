| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.999x | 1.355x |
| materialize | 1.001x | 0.996x | 1.243x |
| perft_bulk_d4 | 1.001x | 0.997x | 1.348x |
| perft_full_d3 | 1.027x | 1.020x | 1.085x |
| fen_parse | 1.007x | 1.000x | 1.247x |
| fen_write_native_ep | 1.000x | 0.994x | 1.568x |
| snapshot | 0.921x | 0.918x | 0.955x |
| san_parse_all | 0.985x | 0.978x | 1.792x |
| san_render_native | 0.996x | 0.984x | 2.548x |
| apply_children_full | 1.101x | 1.045x | 1.148x |
| make_unmake_full | 0.989x | 0.973x | 1.418x |
| startpos | 1.074x | 1.074x | 1.335x |
| chess960_materialize | 0.995x | 0.995x | 1.325x |
| chess960_perft_d3 | 1.006x | 0.999x | 1.330x |
| polyglot_key | 1.016x | 0.999x | 25.555x |
| polyglot_recompute | 1.000x | 0.999x | 1.072x |
| repetition_key | 0.957x | 0.921x | 18.163x |
| corpus_count | 1.008x | 0.999x | 1.467x |
| corpus_materialize | 1.012x | 1.008x | 1.214x |
| corpus_uci_sorted | 1.010x | 1.009x | 1.460x |
| replay_trusted_native | 0.995x | 0.986x | 0.497x |
| replay_checked_native | 1.006x | 1.005x | 0.990x |

Cold first legal move list: 0.010 ms -> 0.011 ms (0.97x).
