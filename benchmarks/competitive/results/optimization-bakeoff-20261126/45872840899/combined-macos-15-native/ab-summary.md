| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.982x | 0.928x | 1.503x |
| materialize | 0.982x | 0.867x | 1.092x |
| perft_bulk_d4 | 0.967x | 0.842x | 1.275x |
| perft_full_d3 | 0.975x | 0.889x | 1.328x |
| fen_parse | 0.960x | 0.821x | 1.185x |
| fen_write_native_ep | 0.987x | 0.965x | 1.795x |
| snapshot | 0.980x | 0.912x | 0.997x |
| san_parse_all | 0.964x | 0.890x | 1.641x |
| san_render_native | 0.966x | 0.930x | 2.830x |
| apply_children_full | 1.067x | 0.952x | 1.171x |
| make_unmake_full | 0.956x | 0.911x | 1.284x |
| startpos | 0.993x | 0.993x | 1.199x |
| chess960_materialize | 0.980x | 0.962x | 1.645x |
| chess960_perft_d3 | 0.978x | 0.978x | 1.505x |
| polyglot_key | 0.990x | 0.962x | 41.872x |
| polyglot_recompute | 0.972x | 0.960x | 1.025x |
| repetition_key | 1.009x | 1.005x | 20.788x |
| corpus_count | 0.978x | 0.961x | 1.271x |
| corpus_materialize | 0.997x | 0.992x | 1.155x |
| corpus_uci_sorted | 0.983x | 0.980x | 1.406x |
| replay_trusted_native | 0.933x | 0.912x | 0.626x |
| replay_checked_native | 0.978x | 0.948x | 1.211x |

Cold first legal move list: 0.003 ms -> 0.003 ms (0.82x).
