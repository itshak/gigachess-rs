| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.993x | 1.081x |
| materialize | 1.002x | 0.997x | 1.182x |
| perft_bulk_d4 | 1.027x | 1.006x | 0.908x |
| perft_full_d3 | 1.000x | 0.998x | 0.597x |
| fen_parse | 0.981x | 0.974x | 1.165x |
| fen_write_native_ep | 0.998x | 0.992x | 1.044x |
| snapshot | 1.023x | 1.013x | 0.705x |
| san_parse_all | 1.027x | 1.014x | 0.945x |
| san_render_native | 1.020x | 1.003x | 1.161x |
| apply_children_full | 1.053x | 1.022x | 0.688x |
| make_unmake_full | 1.067x | 1.028x | 0.960x |
| startpos | 1.039x | 1.039x | 0.802x |
| chess960_materialize | 1.002x | 1.002x | 1.105x |
| chess960_perft_d3 | 1.001x | 0.998x | 0.842x |
| polyglot_key | 0.973x | 0.953x | 22.708x |
| polyglot_recompute | 0.996x | 0.992x | 1.126x |
| repetition_key | 0.988x | 0.980x | 13.744x |
| corpus_count | 0.991x | 0.990x | 1.199x |
| corpus_materialize | 0.997x | 0.995x | 1.125x |
| corpus_uci_sorted | 0.995x | 0.994x | 0.567x |
| replay_trusted_native | 1.094x | 1.058x | 0.345x |
| replay_checked_native | 1.046x | 1.030x | 0.689x |

Cold first legal move list: 154.245 ms → 154.023 ms (1.00x).
