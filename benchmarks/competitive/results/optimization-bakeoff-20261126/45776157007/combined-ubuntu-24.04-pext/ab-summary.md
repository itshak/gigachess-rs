| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.998x | 1.427x |
| materialize | 1.088x | 1.017x | 1.496x |
| perft_bulk_d4 | 0.996x | 0.978x | 1.239x |
| perft_full_d3 | 0.969x | 0.966x | 0.869x |
| fen_parse | 1.023x | 1.002x | 1.062x |
| fen_write_native_ep | 0.993x | 0.963x | 1.223x |
| snapshot | 1.095x | 1.081x | 0.996x |
| san_parse_all | 0.930x | 0.915x | 1.659x |
| san_render_native | 1.009x | 0.989x | 2.120x |
| apply_children_full | 1.011x | 0.983x | 1.002x |
| make_unmake_full | 0.993x | 0.984x | 0.948x |
| startpos | 0.892x | 0.892x | 1.092x |
| chess960_materialize | 0.998x | 0.992x | 1.397x |
| chess960_perft_d3 | 0.997x | 0.989x | 1.321x |
| polyglot_key | 0.989x | 0.978x | 39.371x |
| polyglot_recompute | 0.999x | 0.999x | 1.022x |
| repetition_key | 0.975x | 0.952x | 19.818x |
| corpus_count | 0.995x | 0.992x | 1.381x |
| corpus_materialize | 1.012x | 1.011x | 1.123x |
| corpus_uci_sorted | 1.610x | 1.603x | 1.556x |
| replay_trusted_native | 0.992x | 0.987x | 0.598x |
| replay_checked_native | 1.006x | 0.999x | 0.998x |

Cold first legal move list: 0.008 ms -> 0.009 ms (0.93x).
