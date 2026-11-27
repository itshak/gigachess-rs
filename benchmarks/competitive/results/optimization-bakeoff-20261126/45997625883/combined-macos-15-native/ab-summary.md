| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.016x | 0.862x | 1.586x |
| materialize | 0.921x | 0.444x | 1.005x |
| perft_bulk_d4 | 0.998x | 0.867x | 1.221x |
| perft_full_d3 | 1.042x | 0.837x | 1.304x |
| fen_parse | 1.003x | 0.815x | 1.176x |
| fen_write_native_ep | 0.968x | 0.745x | 1.762x |
| snapshot | 1.080x | 0.805x | 1.010x |
| san_parse_all | 0.997x | 0.872x | 1.752x |
| san_render_native | 1.045x | 0.777x | 2.958x |
| apply_children_full | 1.052x | 0.806x | 1.117x |
| make_unmake_full | 1.037x | 0.948x | 1.419x |
| startpos | 0.688x | 0.688x | 1.270x |
| chess960_materialize | 1.169x | 0.979x | 1.636x |
| chess960_perft_d3 | 1.057x | 0.902x | 1.719x |
| polyglot_key | 1.204x | 1.183x | 36.674x |
| polyglot_recompute | 0.981x | 0.967x | 0.980x |
| repetition_key | 0.991x | 0.961x | 18.213x |
| corpus_count | 1.076x | 0.975x | 1.491x |
| corpus_materialize | 1.171x | 1.142x | 1.277x |
| corpus_uci_sorted | 1.135x | 1.015x | 1.578x |
| replay_trusted_native | 1.014x | 0.994x | 0.646x |
| replay_checked_native | 1.155x | 1.131x | 1.257x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.18x).
