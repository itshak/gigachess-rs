| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 0.922x | 1.381x |
| materialize | 1.017x | 0.927x | 1.080x |
| perft_bulk_d4 | 0.997x | 0.903x | 1.342x |
| perft_full_d3 | 0.978x | 0.935x | 1.058x |
| fen_parse | 1.073x | 1.038x | 1.285x |
| fen_write_native_ep | 0.998x | 0.992x | 1.615x |
| snapshot | 0.718x | 0.698x | 0.809x |
| san_parse_all | 0.993x | 0.953x | 1.875x |
| san_render_native | 0.997x | 0.947x | 3.426x |
| apply_children_full | 0.986x | 0.906x | 1.028x |
| make_unmake_full | 0.957x | 0.918x | 1.209x |
| startpos | 1.099x | 1.099x | 1.183x |
| chess960_materialize | 1.036x | 0.998x | 1.259x |
| chess960_perft_d3 | 1.009x | 0.937x | 1.499x |
| polyglot_key | 1.003x | 1.000x | 20.433x |
| polyglot_recompute | 0.979x | 0.952x | 1.016x |
| repetition_key | 0.912x | 0.855x | 17.158x |
| corpus_count | 0.845x | 0.777x | 1.139x |
| corpus_materialize | 0.952x | 0.941x | 1.139x |
| corpus_uci_sorted | 0.972x | 0.965x | 1.556x |
| replay_trusted_native | 0.948x | 0.946x | 0.727x |
| replay_checked_native | 0.922x | 0.885x | 0.864x |

Cold first legal move list: 0.023 ms -> 0.013 ms (1.76x).
