| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.975x | 0.890x | 1.282x |
| materialize | 0.997x | 0.976x | 1.162x |
| perft_bulk_d4 | 0.978x | 0.873x | 1.236x |
| perft_full_d3 | 0.901x | 0.844x | 0.886x |
| fen_parse | 0.966x | 0.900x | 1.186x |
| fen_write_native_ep | 0.992x | 0.981x | 1.542x |
| snapshot | 1.143x | 1.103x | 1.383x |
| san_parse_all | 0.988x | 0.949x | 1.930x |
| san_render_native | 1.175x | 1.117x | 3.918x |
| apply_children_full | 1.336x | 1.271x | 1.201x |
| make_unmake_full | 0.933x | 0.824x | 1.234x |
| startpos | 0.394x | 0.394x | 0.500x |
| chess960_materialize | 1.004x | 0.985x | 1.165x |
| chess960_perft_d3 | 0.981x | 0.966x | 1.411x |
| polyglot_key | 0.987x | 0.967x | 37.406x |
| polyglot_recompute | 0.966x | 0.950x | 0.971x |
| repetition_key | 1.064x | 0.997x | 26.181x |
| corpus_count | 0.967x | 0.951x | 1.189x |
| corpus_materialize | 1.050x | 0.985x | 1.207x |
| corpus_uci_sorted | 0.988x | 0.982x | 1.390x |
| replay_trusted_native | 0.971x | 0.964x | 0.599x |
| replay_checked_native | 0.989x | 0.978x | 0.933x |

Cold first legal move list: 0.026 ms -> 0.022 ms (1.14x).
