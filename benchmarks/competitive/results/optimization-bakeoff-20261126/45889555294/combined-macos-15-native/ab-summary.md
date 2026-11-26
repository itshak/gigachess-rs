| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.943x | 0.846x | 1.422x |
| materialize | 0.948x | 0.857x | 1.080x |
| perft_bulk_d4 | 0.921x | 0.796x | 1.200x |
| perft_full_d3 | 0.755x | 0.656x | 0.976x |
| fen_parse | 1.056x | 0.743x | 1.185x |
| fen_write_native_ep | 0.958x | 0.764x | 1.612x |
| snapshot | 1.167x | 0.859x | 1.201x |
| san_parse_all | 1.124x | 0.902x | 1.769x |
| san_render_native | 1.088x | 1.000x | 3.100x |
| apply_children_full | 1.103x | 0.893x | 1.266x |
| make_unmake_full | 0.932x | 0.709x | 1.113x |
| startpos | 2.490x | 2.490x | 2.576x |
| chess960_materialize | 1.090x | 0.985x | 1.507x |
| chess960_perft_d3 | 0.941x | 0.845x | 1.353x |
| polyglot_key | 1.126x | 1.105x | 41.184x |
| polyglot_recompute | 1.008x | 0.978x | 1.009x |
| repetition_key | 1.219x | 1.190x | 21.750x |
| corpus_count | 1.060x | 1.022x | 1.060x |
| corpus_materialize | 1.084x | 1.023x | 1.139x |
| corpus_uci_sorted | 1.123x | 1.057x | 1.349x |
| replay_trusted_native | 1.084x | 1.072x | 0.618x |
| replay_checked_native | 1.370x | 1.210x | 1.207x |

Cold first legal move list: 0.008 ms -> 0.005 ms (1.43x).
