| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.978x | 0.972x | 1.325x |
| materialize | 1.004x | 1.000x | 1.251x |
| perft_bulk_d4 | 0.801x | 0.768x | 1.079x |
| perft_full_d3 | 0.774x | 0.728x | 0.817x |
| fen_parse | 0.963x | 0.938x | 1.201x |
| fen_write_native_ep | 0.973x | 0.952x | 1.539x |
| snapshot | 1.399x | 1.380x | 1.319x |
| san_parse_all | 0.990x | 0.957x | 1.851x |
| san_render_native | 0.724x | 0.657x | 1.868x |
| apply_children_full | 0.690x | 0.635x | 0.720x |
| make_unmake_full | 0.527x | 0.474x | 0.755x |
| startpos | 0.964x | 0.964x | 1.143x |
| chess960_materialize | 0.986x | 0.982x | 1.315x |
| chess960_perft_d3 | 0.782x | 0.739x | 1.045x |
| polyglot_key | 1.100x | 0.929x | 28.473x |
| polyglot_recompute | 0.977x | 0.976x | 1.050x |
| repetition_key | 1.231x | 1.099x | 23.407x |
| corpus_count | 0.981x | 0.971x | 1.406x |
| corpus_materialize | 1.004x | 1.001x | 1.210x |
| corpus_uci_sorted | 1.004x | 0.999x | 1.459x |
| replay_trusted_native | 0.788x | 0.729x | 0.395x |
| replay_checked_native | 0.745x | 0.739x | 0.735x |

Cold first legal move list: 0.011 ms -> 0.010 ms (1.10x).
