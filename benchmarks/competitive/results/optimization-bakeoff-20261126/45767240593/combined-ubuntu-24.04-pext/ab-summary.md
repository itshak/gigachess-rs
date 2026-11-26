| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.117x | 1.073x | 1.379x |
| materialize | 1.060x | 1.026x | 1.507x |
| perft_bulk_d4 | 1.270x | 1.222x | 1.117x |
| perft_full_d3 | 1.375x | 1.296x | 0.817x |
| fen_parse | 1.038x | 0.985x | 1.109x |
| fen_write_native_ep | 1.021x | 1.001x | 1.344x |
| snapshot | 2.115x | 2.096x | 1.149x |
| san_parse_all | 2.005x | 1.935x | 1.854x |
| san_render_native | 2.027x | 1.848x | 2.095x |
| apply_children_full | 1.594x | 1.450x | 0.974x |
| make_unmake_full | 1.272x | 1.177x | 0.943x |
| startpos | 1.664x | 1.664x | 0.830x |
| chess960_materialize | 1.171x | 1.097x | 1.400x |
| chess960_perft_d3 | 1.345x | 1.254x | 1.239x |
| polyglot_key | 1.189x | 1.110x | 39.785x |
| polyglot_recompute | 1.033x | 1.023x | 1.021x |
| repetition_key | 1.328x | 1.204x | 21.734x |
| corpus_count | 1.186x | 1.121x | 1.364x |
| corpus_materialize | 1.033x | 1.031x | 1.123x |
| corpus_uci_sorted | 1.247x | 1.247x | 0.765x |
| replay_trusted_native | 1.254x | 1.134x | 0.607x |
| replay_checked_native | 1.344x | 1.147x | 0.972x |

Cold first legal move list: 0.989 ms → 0.009 ms (111.68x).
