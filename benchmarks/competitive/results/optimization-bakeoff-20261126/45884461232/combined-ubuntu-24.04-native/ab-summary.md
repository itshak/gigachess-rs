| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.033x | 0.957x | 1.443x |
| materialize | 0.990x | 0.970x | 1.215x |
| perft_bulk_d4 | 0.982x | 0.939x | 1.250x |
| perft_full_d3 | 0.999x | 0.976x | 1.234x |
| fen_parse | 0.993x | 0.977x | 1.237x |
| fen_write_native_ep | 0.977x | 0.908x | 1.455x |
| snapshot | 0.990x | 0.976x | 1.492x |
| san_parse_all | 0.987x | 0.967x | 1.772x |
| san_render_native | 0.972x | 0.936x | 2.248x |
| apply_children_full | 0.890x | 0.816x | 1.243x |
| make_unmake_full | 0.946x | 0.866x | 1.169x |
| startpos | 1.000x | 1.000x | 1.246x |
| chess960_materialize | 0.997x | 0.971x | 1.524x |
| chess960_perft_d3 | 0.990x | 0.985x | 1.501x |
| polyglot_key | 0.975x | 0.963x | 35.123x |
| polyglot_recompute | 0.996x | 0.991x | 0.987x |
| repetition_key | 0.982x | 0.966x | 27.832x |
| corpus_count | 0.979x | 0.969x | 1.118x |
| corpus_materialize | 1.005x | 0.994x | 1.145x |
| corpus_uci_sorted | 1.043x | 0.992x | 1.700x |
| replay_trusted_native | 1.068x | 0.997x | 0.811x |
| replay_checked_native | 0.951x | 0.943x | 1.150x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.03x).
