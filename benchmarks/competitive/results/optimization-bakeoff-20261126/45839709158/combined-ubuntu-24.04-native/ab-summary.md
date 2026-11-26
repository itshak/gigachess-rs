| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.976x | 1.466x |
| materialize | 1.005x | 1.000x | 1.258x |
| perft_bulk_d4 | 1.026x | 1.015x | 1.377x |
| perft_full_d3 | 0.976x | 0.903x | 1.091x |
| fen_parse | 1.040x | 1.030x | 1.278x |
| fen_write_native_ep | 1.003x | 0.997x | 1.800x |
| snapshot | 1.016x | 0.998x | 1.247x |
| san_parse_all | 1.026x | 1.011x | 2.043x |
| san_render_native | 0.992x | 0.983x | 2.196x |
| apply_children_full | 1.022x | 0.992x | 1.320x |
| make_unmake_full | 1.035x | 1.014x | 1.310x |
| startpos | 0.999x | 0.999x | 1.250x |
| chess960_materialize | 1.005x | 0.997x | 1.428x |
| chess960_perft_d3 | 0.997x | 0.997x | 1.528x |
| polyglot_key | 0.991x | 0.985x | 29.427x |
| polyglot_recompute | 1.031x | 0.995x | 1.061x |
| repetition_key | 0.993x | 0.985x | 20.290x |
| corpus_count | 1.006x | 0.997x | 1.387x |
| corpus_materialize | 1.001x | 0.997x | 1.193x |
| corpus_uci_sorted | 1.012x | 1.008x | 1.854x |
| replay_trusted_native | 0.992x | 0.978x | 0.895x |
| replay_checked_native | 1.000x | 0.998x | 1.029x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.18x).
