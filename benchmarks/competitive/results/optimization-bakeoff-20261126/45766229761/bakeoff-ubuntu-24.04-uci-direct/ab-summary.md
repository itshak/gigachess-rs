| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.997x | 1.199x |
| materialize | 1.010x | 0.996x | 1.265x |
| perft_bulk_d4 | 0.999x | 0.996x | 0.886x |
| perft_full_d3 | 1.004x | 1.001x | 0.579x |
| fen_parse | 1.059x | 1.038x | 1.186x |
| fen_write_native_ep | 1.014x | 0.981x | 1.379x |
| snapshot | 1.143x | 1.139x | 0.714x |
| san_parse_all | 1.009x | 0.996x | 0.863x |
| san_render_native | 0.991x | 0.984x | 1.159x |
| apply_children_full | 1.042x | 1.030x | 0.637x |
| make_unmake_full | 0.992x | 0.965x | 0.801x |
| startpos | 0.893x | 0.893x | 0.528x |
| chess960_materialize | 0.981x | 0.974x | 1.083x |
| chess960_perft_d3 | 0.994x | 0.986x | 0.872x |
| polyglot_key | 0.858x | 0.757x | 24.807x |
| polyglot_recompute | 0.982x | 0.955x | 1.093x |
| repetition_key | 1.003x | 0.996x | 14.299x |
| corpus_count | 1.000x | 0.999x | 1.150x |
| corpus_materialize | 0.999x | 0.999x | 1.107x |
| corpus_uci_sorted | 1.554x | 1.552x | 0.975x |
| replay_trusted_native | 1.021x | 1.020x | 0.484x |
| replay_checked_native | 0.998x | 0.993x | 0.716x |

Cold first legal move list: 183.311 ms → 193.685 ms (0.95x).
