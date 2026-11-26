| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.978x | 0.939x | 1.295x |
| materialize | 1.026x | 0.988x | 1.240x |
| perft_bulk_d4 | 0.968x | 0.923x | 1.168x |
| perft_full_d3 | 0.916x | 0.864x | 1.131x |
| fen_parse | 1.078x | 1.045x | 1.291x |
| fen_write_native_ep | 0.984x | 0.953x | 1.643x |
| snapshot | 1.000x | 0.996x | 1.250x |
| san_parse_all | 0.991x | 0.982x | 2.069x |
| san_render_native | 0.974x | 0.944x | 2.169x |
| apply_children_full | 0.997x | 0.966x | 1.277x |
| make_unmake_full | 0.945x | 0.924x | 1.194x |
| startpos | 1.508x | 1.508x | 1.875x |
| chess960_materialize | 0.983x | 0.974x | 1.320x |
| chess960_perft_d3 | 0.956x | 0.952x | 1.325x |
| polyglot_key | 0.927x | 0.906x | 28.069x |
| polyglot_recompute | 0.970x | 0.970x | 1.018x |
| repetition_key | 1.027x | 1.007x | 21.768x |
| corpus_count | 0.981x | 0.970x | 1.259x |
| corpus_materialize | 1.021x | 1.013x | 1.136x |
| corpus_uci_sorted | 1.059x | 1.056x | 1.874x |
| replay_trusted_native | 0.851x | 0.797x | 0.744x |
| replay_checked_native | 0.990x | 0.977x | 0.995x |

Cold first legal move list: 0.008 ms -> 0.008 ms (0.97x).
