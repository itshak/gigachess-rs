| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.039x | 1.020x | 1.372x |
| materialize | 1.006x | 0.983x | 1.123x |
| perft_bulk_d4 | 0.989x | 0.927x | 1.342x |
| perft_full_d3 | 1.609x | 1.498x | 1.039x |
| fen_parse | 0.785x | 0.721x | 1.212x |
| fen_write_native_ep | 0.988x | 0.982x | 1.218x |
| snapshot | 0.323x | 0.311x | 0.945x |
| san_parse_all | 1.085x | 1.034x | 1.916x |
| san_render_native | 1.025x | 0.856x | 3.438x |
| apply_children_full | 1.748x | 1.376x | 1.047x |
| make_unmake_full | 1.370x | 1.290x | 1.247x |
| startpos | 0.981x | 0.981x | 2.745x |
| chess960_materialize | 1.014x | 0.987x | 1.213x |
| chess960_perft_d3 | 0.969x | 0.960x | 1.363x |
| polyglot_key | 0.987x | 0.966x | 20.057x |
| polyglot_recompute | 0.964x | 0.902x | 0.924x |
| repetition_key | 0.944x | 0.900x | 16.830x |
| corpus_count | 1.007x | 1.006x | 1.339x |
| corpus_materialize | 0.980x | 0.963x | 1.185x |
| corpus_uci_sorted | 1.404x | 1.302x | 1.396x |
| replay_trusted_native | 1.477x | 1.399x | 0.762x |
| replay_checked_native | 1.038x | 0.991x | 0.931x |

Cold first legal move list: 0.026 ms -> 0.026 ms (0.98x).
