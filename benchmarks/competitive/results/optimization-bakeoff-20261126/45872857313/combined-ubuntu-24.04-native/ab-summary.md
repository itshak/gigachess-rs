| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.931x | 1.510x |
| materialize | 0.996x | 0.969x | 1.567x |
| perft_bulk_d4 | 1.005x | 0.954x | 1.161x |
| perft_full_d3 | 1.009x | 0.969x | 1.508x |
| fen_parse | 0.986x | 0.975x | 1.101x |
| fen_write_native_ep | 1.006x | 0.916x | 2.066x |
| snapshot | 1.006x | 0.949x | 0.987x |
| san_parse_all | 1.025x | 0.997x | 1.942x |
| san_render_native | 0.954x | 0.908x | 2.172x |
| apply_children_full | 0.988x | 0.953x | 1.370x |
| make_unmake_full | 0.945x | 0.904x | 1.156x |
| startpos | 0.807x | 0.807x | 1.006x |
| chess960_materialize | 0.966x | 0.943x | 1.528x |
| chess960_perft_d3 | 1.039x | 1.011x | 1.516x |
| polyglot_key | 1.007x | 0.962x | 38.151x |
| polyglot_recompute | 1.080x | 1.069x | 1.086x |
| repetition_key | 1.137x | 0.976x | 22.362x |
| corpus_count | 1.033x | 0.999x | 1.123x |
| corpus_materialize | 1.035x | 1.025x | 1.086x |
| corpus_uci_sorted | 1.027x | 0.977x | 1.500x |
| replay_trusted_native | 1.029x | 1.018x | 0.895x |
| replay_checked_native | 0.982x | 0.965x | 1.148x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.83x).
