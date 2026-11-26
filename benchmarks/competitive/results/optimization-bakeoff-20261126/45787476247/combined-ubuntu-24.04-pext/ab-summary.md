| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.997x | 1.438x |
| materialize | 1.003x | 0.978x | 1.162x |
| perft_bulk_d4 | 1.031x | 0.997x | 1.330x |
| perft_full_d3 | 0.994x | 0.984x | 0.976x |
| fen_parse | 0.950x | 0.884x | 1.098x |
| fen_write_native_ep | 0.994x | 0.982x | 1.886x |
| snapshot | 1.002x | 0.994x | 1.111x |
| san_parse_all | 0.998x | 0.988x | 2.231x |
| san_render_native | 0.861x | 0.798x | 1.846x |
| apply_children_full | 0.854x | 0.828x | 0.892x |
| make_unmake_full | 0.964x | 0.917x | 1.149x |
| startpos | 1.000x | 1.000x | 1.183x |
| chess960_materialize | 0.991x | 0.978x | 1.481x |
| chess960_perft_d3 | 1.000x | 0.988x | 1.652x |
| polyglot_key | 0.999x | 0.998x | 20.474x |
| polyglot_recompute | 0.997x | 0.995x | 0.982x |
| repetition_key | 1.001x | 1.000x | 18.986x |
| corpus_count | 1.010x | 1.006x | 1.457x |
| corpus_materialize | 1.010x | 1.009x | 1.283x |
| corpus_uci_sorted | 0.993x | 0.984x | 1.658x |
| replay_trusted_native | 1.002x | 0.986x | 0.750x |
| replay_checked_native | 0.983x | 0.979x | 0.925x |

Cold first legal move list: 0.012 ms -> 0.012 ms (0.98x).
