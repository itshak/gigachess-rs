| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.931x | 0.782x | 1.589x |
| materialize | 0.917x | 0.729x | 1.056x |
| perft_bulk_d4 | 1.090x | 0.955x | 1.294x |
| perft_full_d3 | 0.805x | 0.578x | 1.151x |
| fen_parse | 0.927x | 0.800x | 1.171x |
| fen_write_native_ep | 0.888x | 0.753x | 1.646x |
| snapshot | 0.979x | 0.689x | 0.923x |
| san_parse_all | 1.067x | 0.950x | 1.890x |
| san_render_native | 0.919x | 0.549x | 2.349x |
| apply_children_full | 1.009x | 0.826x | 1.053x |
| make_unmake_full | 0.982x | 0.867x | 1.234x |
| startpos | 0.994x | 0.994x | 2.131x |
| chess960_materialize | 0.916x | 0.855x | 1.675x |
| chess960_perft_d3 | 0.924x | 0.816x | 1.621x |
| polyglot_key | 0.905x | 0.832x | 40.081x |
| polyglot_recompute | 0.915x | 0.865x | 1.122x |
| repetition_key | 0.897x | 0.877x | 20.289x |
| corpus_count | 0.717x | 0.600x | 0.831x |
| corpus_materialize | 1.020x | 1.006x | 1.213x |
| corpus_uci_sorted | 1.003x | 0.854x | 1.337x |
| replay_trusted_native | 0.893x | 0.840x | 0.586x |
| replay_checked_native | 0.918x | 0.913x | 1.186x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.95x).
