| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.915x | 0.601x | 1.314x |
| materialize | 0.933x | 0.817x | 1.036x |
| perft_bulk_d4 | 1.110x | 0.983x | 1.354x |
| perft_full_d3 | 1.068x | 0.809x | 1.444x |
| fen_parse | 0.985x | 0.907x | 1.242x |
| fen_write_native_ep | 0.974x | 0.769x | 1.776x |
| snapshot | 0.863x | 0.654x | 0.872x |
| san_parse_all | 0.904x | 0.674x | 1.704x |
| san_render_native | 0.927x | 0.805x | 2.708x |
| apply_children_full | 1.034x | 0.828x | 1.161x |
| make_unmake_full | 0.980x | 0.900x | 1.343x |
| startpos | 1.066x | 1.066x | 1.162x |
| chess960_materialize | 1.008x | 0.943x | 1.537x |
| chess960_perft_d3 | 1.043x | 1.029x | 1.515x |
| polyglot_key | 0.971x | 0.911x | 41.687x |
| polyglot_recompute | 0.800x | 0.728x | 0.845x |
| repetition_key | 0.902x | 0.862x | 19.386x |
| corpus_count | 0.981x | 0.806x | 1.255x |
| corpus_materialize | 0.954x | 0.824x | 1.159x |
| corpus_uci_sorted | 0.918x | 0.826x | 1.521x |
| replay_trusted_native | 0.827x | 0.769x | 0.679x |
| replay_checked_native | 0.939x | 0.930x | 1.448x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.85x).
