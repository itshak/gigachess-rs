| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.982x | 0.973x | 1.464x |
| materialize | 1.009x | 0.999x | 1.182x |
| perft_bulk_d4 | 0.988x | 0.972x | 1.341x |
| perft_full_d3 | 0.932x | 0.832x | 1.215x |
| fen_parse | 0.964x | 0.938x | 1.301x |
| fen_write_native_ep | 0.982x | 0.971x | 1.516x |
| snapshot | 0.987x | 0.976x | 1.501x |
| san_parse_all | 1.005x | 0.987x | 1.969x |
| san_render_native | 0.988x | 0.974x | 3.809x |
| apply_children_full | 0.999x | 0.990x | 1.552x |
| make_unmake_full | 0.993x | 0.945x | 1.258x |
| startpos | 0.946x | 0.946x | 0.989x |
| chess960_materialize | 0.996x | 0.991x | 1.390x |
| chess960_perft_d3 | 0.989x | 0.976x | 1.472x |
| polyglot_key | 0.992x | 0.986x | 36.375x |
| polyglot_recompute | 1.027x | 0.971x | 0.973x |
| repetition_key | 1.059x | 0.972x | 27.887x |
| corpus_count | 0.959x | 0.956x | 1.135x |
| corpus_materialize | 0.991x | 0.985x | 1.058x |
| corpus_uci_sorted | 0.986x | 0.985x | 1.399x |
| replay_trusted_native | 1.081x | 1.001x | 0.770x |
| replay_checked_native | 0.966x | 0.953x | 1.141x |

Cold first legal move list: 0.012 ms -> 0.010 ms (1.20x).
