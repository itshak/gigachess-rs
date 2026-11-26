| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.050x | 1.022x | 1.198x |
| materialize | 1.048x | 1.019x | 1.277x |
| perft_bulk_d4 | 1.047x | 1.037x | 1.237x |
| perft_full_d3 | 1.581x | 1.522x | 1.371x |
| fen_parse | 0.952x | 0.934x | 1.200x |
| fen_write_native_ep | 0.978x | 0.972x | 0.905x |
| snapshot | 1.001x | 0.997x | 1.000x |
| san_parse_all | 1.073x | 1.047x | 2.060x |
| san_render_native | 1.001x | 0.966x | 2.360x |
| apply_children_full | 1.290x | 1.212x | 1.159x |
| make_unmake_full | 1.246x | 1.211x | 1.330x |
| startpos | 1.030x | 1.030x | 1.265x |
| chess960_materialize | 1.046x | 1.042x | 1.342x |
| chess960_perft_d3 | 1.093x | 1.075x | 1.412x |
| polyglot_key | 1.067x | 0.985x | 27.377x |
| polyglot_recompute | 1.008x | 0.999x | 1.029x |
| repetition_key | 1.060x | 1.012x | 20.859x |
| corpus_count | 1.048x | 1.048x | 1.278x |
| corpus_materialize | 0.988x | 0.988x | 1.183x |
| corpus_uci_sorted | 1.809x | 1.770x | 1.806x |
| replay_trusted_native | 1.198x | 1.166x | 0.702x |
| replay_checked_native | 1.064x | 1.050x | 1.035x |

Cold first legal move list: 0.015 ms -> 0.013 ms (1.18x).
