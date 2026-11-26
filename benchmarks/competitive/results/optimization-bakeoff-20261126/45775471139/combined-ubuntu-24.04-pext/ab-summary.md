| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 0.993x | 1.432x |
| materialize | 0.934x | 0.897x | 1.429x |
| perft_bulk_d4 | 0.948x | 0.934x | 1.175x |
| perft_full_d3 | 0.991x | 0.954x | 0.871x |
| fen_parse | 0.986x | 0.954x | 1.170x |
| fen_write_native_ep | 0.959x | 0.926x | 1.173x |
| snapshot | 0.865x | 0.860x | 0.869x |
| san_parse_all | 0.933x | 0.904x | 1.751x |
| san_render_native | 1.002x | 0.984x | 2.103x |
| apply_children_full | 0.956x | 0.942x | 0.963x |
| make_unmake_full | 0.934x | 0.919x | 0.885x |
| startpos | 1.123x | 1.123x | 1.206x |
| chess960_materialize | 1.006x | 0.975x | 1.335x |
| chess960_perft_d3 | 0.992x | 0.982x | 1.304x |
| polyglot_key | 0.979x | 0.961x | 39.997x |
| polyglot_recompute | 0.947x | 0.942x | 1.000x |
| repetition_key | 0.965x | 0.949x | 19.343x |
| corpus_count | 0.983x | 0.982x | 1.364x |
| corpus_materialize | 0.998x | 0.996x | 1.103x |
| corpus_uci_sorted | 0.984x | 0.976x | 0.950x |
| replay_trusted_native | 0.977x | 0.966x | 0.605x |
| replay_checked_native | 0.944x | 0.906x | 0.930x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.91x).
