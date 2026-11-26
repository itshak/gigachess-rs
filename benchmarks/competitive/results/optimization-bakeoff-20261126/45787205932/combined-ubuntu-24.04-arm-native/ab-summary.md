| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.355x |
| materialize | 1.005x | 0.999x | 1.244x |
| perft_bulk_d4 | 0.999x | 0.996x | 1.345x |
| perft_full_d3 | 1.010x | 0.996x | 1.065x |
| fen_parse | 0.998x | 0.996x | 1.238x |
| fen_write_native_ep | 1.001x | 0.998x | 1.571x |
| snapshot | 1.080x | 1.077x | 0.921x |
| san_parse_all | 0.999x | 0.994x | 1.825x |
| san_render_native | 1.001x | 0.996x | 2.574x |
| apply_children_full | 1.038x | 1.031x | 1.080x |
| make_unmake_full | 1.007x | 1.001x | 1.453x |
| startpos | 1.038x | 1.038x | 1.185x |
| chess960_materialize | 0.995x | 0.990x | 1.321x |
| chess960_perft_d3 | 0.995x | 0.992x | 1.319x |
| polyglot_key | 0.987x | 0.975x | 25.085x |
| polyglot_recompute | 1.003x | 1.001x | 1.078x |
| repetition_key | 0.977x | 0.955x | 18.512x |
| corpus_count | 0.995x | 0.994x | 1.501x |
| corpus_materialize | 0.992x | 0.987x | 1.219x |
| corpus_uci_sorted | 1.000x | 0.998x | 1.451x |
| replay_trusted_native | 1.003x | 0.998x | 0.514x |
| replay_checked_native | 1.008x | 1.005x | 0.990x |

Cold first legal move list: 0.012 ms -> 0.012 ms (0.97x).
