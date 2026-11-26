| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.983x | 0.945x | 1.110x |
| materialize | 0.904x | 0.850x | 1.207x |
| perft_bulk_d4 | 1.008x | 0.977x | 1.161x |
| perft_full_d3 | 0.953x | 0.922x | 0.706x |
| fen_parse | 0.928x | 0.888x | 1.160x |
| fen_write_native_ep | 0.995x | 0.988x | 1.210x |
| snapshot | 1.126x | 1.058x | 1.021x |
| san_parse_all | 0.981x | 0.962x | 1.964x |
| san_render_native | 1.023x | 0.986x | 2.348x |
| apply_children_full | 1.016x | 0.973x | 0.860x |
| make_unmake_full | 0.938x | 0.917x | 0.948x |
| startpos | 1.077x | 1.077x | 1.384x |
| chess960_materialize | 0.978x | 0.960x | 1.275x |
| chess960_perft_d3 | 0.970x | 0.967x | 1.329x |
| polyglot_key | 1.034x | 1.001x | 22.621x |
| polyglot_recompute | 0.973x | 0.971x | 0.954x |
| repetition_key | 0.986x | 0.971x | 20.027x |
| corpus_count | 0.961x | 0.955x | 1.156x |
| corpus_materialize | 0.986x | 0.978x | 1.332x |
| corpus_uci_sorted | 0.990x | 0.987x | 0.963x |
| replay_trusted_native | 0.958x | 0.957x | 0.533x |
| replay_checked_native | 0.915x | 0.911x | 0.954x |

Cold first legal move list: 0.013 ms -> 0.012 ms (1.04x).
