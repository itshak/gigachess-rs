| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.040x | 0.966x | 1.475x |
| materialize | 0.999x | 0.975x | 1.159x |
| perft_bulk_d4 | 1.001x | 0.920x | 1.321x |
| perft_full_d3 | 1.063x | 1.015x | 1.328x |
| fen_parse | 0.908x | 0.858x | 1.214x |
| fen_write_native_ep | 1.009x | 0.999x | 1.544x |
| snapshot | 0.993x | 0.960x | 1.488x |
| san_parse_all | 1.009x | 0.988x | 2.018x |
| san_render_native | 1.001x | 0.975x | 4.030x |
| apply_children_full | 1.011x | 0.992x | 1.394x |
| make_unmake_full | 1.024x | 0.999x | 1.318x |
| startpos | 0.978x | 0.978x | 1.006x |
| chess960_materialize | 0.997x | 0.989x | 1.384x |
| chess960_perft_d3 | 0.984x | 0.953x | 1.466x |
| polyglot_key | 1.011x | 0.998x | 36.646x |
| polyglot_recompute | 1.216x | 0.997x | 1.148x |
| repetition_key | 1.001x | 0.974x | 27.380x |
| corpus_count | 1.006x | 1.003x | 1.177x |
| corpus_materialize | 1.031x | 0.998x | 1.133x |
| corpus_uci_sorted | 0.996x | 0.983x | 1.387x |
| replay_trusted_native | 0.994x | 0.993x | 0.723x |
| replay_checked_native | 1.011x | 0.985x | 1.213x |

Cold first legal move list: 0.012 ms -> 0.013 ms (0.97x).
