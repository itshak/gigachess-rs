| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.038x | 0.984x | 1.529x |
| materialize | 0.986x | 0.938x | 1.234x |
| perft_bulk_d4 | 1.008x | 0.897x | 1.339x |
| perft_full_d3 | 0.968x | 0.825x | 1.073x |
| fen_parse | 1.070x | 0.917x | 1.187x |
| fen_write_native_ep | 0.935x | 0.797x | 1.595x |
| snapshot | 0.963x | 0.815x | 0.975x |
| san_parse_all | 0.983x | 0.895x | 2.142x |
| san_render_native | 1.016x | 0.997x | 3.892x |
| apply_children_full | 0.909x | 0.781x | 1.260x |
| make_unmake_full | 0.917x | 0.872x | 1.202x |
| startpos | 1.005x | 1.005x | 1.251x |
| chess960_materialize | 1.075x | 0.975x | 1.337x |
| chess960_perft_d3 | 1.002x | 0.997x | 1.542x |
| polyglot_key | 1.001x | 1.001x | 25.712x |
| polyglot_recompute | 0.615x | 0.595x | 0.614x |
| repetition_key | 0.980x | 0.961x | 20.335x |
| corpus_count | 1.007x | 0.992x | 1.397x |
| corpus_materialize | 1.025x | 1.012x | 1.203x |
| corpus_uci_sorted | 0.993x | 0.977x | 1.402x |
| replay_trusted_native | 1.016x | 0.999x | 0.848x |
| replay_checked_native | 0.949x | 0.940x | 0.977x |

Cold first legal move list: 0.022 ms -> 0.019 ms (1.11x).
