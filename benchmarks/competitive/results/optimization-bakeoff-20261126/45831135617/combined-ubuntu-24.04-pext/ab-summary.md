| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.960x | 0.932x | 1.319x |
| materialize | 0.969x | 0.936x | 1.452x |
| perft_bulk_d4 | 0.954x | 0.869x | 1.018x |
| perft_full_d3 | 0.947x | 0.904x | 1.356x |
| fen_parse | 1.063x | 1.004x | 1.151x |
| fen_write_native_ep | 0.947x | 0.869x | 1.768x |
| snapshot | 0.602x | 0.599x | 0.999x |
| san_parse_all | 1.038x | 1.020x | 1.897x |
| san_render_native | 0.998x | 0.960x | 2.102x |
| apply_children_full | 0.991x | 0.970x | 1.294x |
| make_unmake_full | 0.936x | 0.747x | 1.099x |
| startpos | 1.491x | 1.491x | 2.333x |
| chess960_materialize | 0.849x | 0.819x | 1.285x |
| chess960_perft_d3 | 0.975x | 0.956x | 1.347x |
| polyglot_key | 0.970x | 0.938x | 40.610x |
| polyglot_recompute | 0.961x | 0.960x | 1.023x |
| repetition_key | 1.191x | 1.056x | 24.175x |
| corpus_count | 0.975x | 0.941x | 0.902x |
| corpus_materialize | 0.976x | 0.975x | 1.022x |
| corpus_uci_sorted | 1.030x | 1.027x | 1.575x |
| replay_trusted_native | 0.963x | 0.938x | 0.879x |
| replay_checked_native | 0.892x | 0.845x | 0.927x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.90x).
