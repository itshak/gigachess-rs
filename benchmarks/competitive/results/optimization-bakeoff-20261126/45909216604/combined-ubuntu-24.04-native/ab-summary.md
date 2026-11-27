| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.028x | 1.002x | 1.487x |
| materialize | 0.978x | 0.947x | 1.241x |
| perft_bulk_d4 | 1.068x | 1.011x | 1.297x |
| perft_full_d3 | 0.994x | 0.956x | 1.271x |
| fen_parse | 0.891x | 0.840x | 1.118x |
| fen_write_native_ep | 0.987x | 0.967x | 1.539x |
| snapshot | 0.991x | 0.972x | 1.487x |
| san_parse_all | 0.994x | 0.980x | 1.758x |
| san_render_native | 1.022x | 0.980x | 2.300x |
| apply_children_full | 0.992x | 0.967x | 1.351x |
| make_unmake_full | 1.024x | 0.994x | 1.222x |
| startpos | 0.993x | 0.993x | 1.257x |
| chess960_materialize | 0.981x | 0.960x | 1.476x |
| chess960_perft_d3 | 1.016x | 1.012x | 1.495x |
| polyglot_key | 1.000x | 0.992x | 36.195x |
| polyglot_recompute | 1.005x | 0.983x | 0.997x |
| repetition_key | 1.010x | 0.981x | 28.150x |
| corpus_count | 1.061x | 1.013x | 1.226x |
| corpus_materialize | 0.996x | 0.990x | 1.130x |
| corpus_uci_sorted | 1.017x | 1.009x | 1.647x |
| replay_trusted_native | 1.024x | 1.014x | 0.771x |
| replay_checked_native | 1.019x | 1.010x | 1.214x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.86x).
