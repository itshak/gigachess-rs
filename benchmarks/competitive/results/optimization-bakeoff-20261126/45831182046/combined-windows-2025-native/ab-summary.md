| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.084x | 1.026x | 1.485x |
| materialize | 1.053x | 0.978x | 1.094x |
| perft_bulk_d4 | 1.021x | 0.940x | 1.360x |
| perft_full_d3 | 1.055x | 1.019x | 1.140x |
| fen_parse | 1.028x | 0.925x | 1.299x |
| fen_write_native_ep | 1.030x | 0.960x | 1.593x |
| snapshot | 1.023x | 0.994x | 1.110x |
| san_parse_all | 1.037x | 0.894x | 1.990x |
| san_render_native | 1.039x | 0.995x | 3.362x |
| apply_children_full | 1.023x | 0.986x | 1.307x |
| make_unmake_full | 1.022x | 0.961x | 1.283x |
| startpos | 1.000x | 1.000x | 1.000x |
| chess960_materialize | 1.089x | 1.073x | 1.462x |
| chess960_perft_d3 | 1.043x | 1.001x | 1.615x |
| polyglot_key | 0.986x | 0.967x | 20.326x |
| polyglot_recompute | 1.000x | 0.998x | 0.965x |
| repetition_key | 1.003x | 0.995x | 19.357x |
| corpus_count | 1.035x | 1.028x | 1.222x |
| corpus_materialize | 1.038x | 1.032x | 1.145x |
| corpus_uci_sorted | 0.980x | 0.960x | 1.485x |
| replay_trusted_native | 1.025x | 1.022x | 0.962x |
| replay_checked_native | 1.077x | 1.053x | 1.101x |

Cold first legal move list: 0.016 ms -> 0.019 ms (0.87x).
