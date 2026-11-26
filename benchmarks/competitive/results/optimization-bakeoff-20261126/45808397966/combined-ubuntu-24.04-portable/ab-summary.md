| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.021x | 0.943x | 1.233x |
| materialize | 0.852x | 0.699x | 1.063x |
| perft_bulk_d4 | 1.009x | 0.945x | 1.232x |
| perft_full_d3 | 0.988x | 0.930x | 1.337x |
| fen_parse | 0.993x | 0.975x | 1.167x |
| fen_write_native_ep | 1.002x | 0.983x | 1.723x |
| snapshot | 1.001x | 0.995x | 1.021x |
| san_parse_all | 1.023x | 1.013x | 2.030x |
| san_render_native | 0.939x | 0.896x | 2.360x |
| apply_children_full | 1.021x | 1.000x | 1.260x |
| make_unmake_full | 1.021x | 1.001x | 1.317x |
| startpos | 1.065x | 1.065x | 1.335x |
| chess960_materialize | 0.949x | 0.949x | 1.264x |
| chess960_perft_d3 | 1.077x | 0.977x | 1.499x |
| polyglot_key | 0.993x | 0.987x | 27.587x |
| polyglot_recompute | 0.970x | 0.969x | 0.998x |
| repetition_key | 0.857x | 0.850x | 18.199x |
| corpus_count | 0.946x | 0.876x | 1.206x |
| corpus_materialize | 0.881x | 0.837x | 1.057x |
| corpus_uci_sorted | 0.978x | 0.976x | 1.773x |
| replay_trusted_native | 1.002x | 0.999x | 0.775x |
| replay_checked_native | 0.999x | 0.986x | 1.055x |

Cold first legal move list: 0.014 ms -> 0.008 ms (1.82x).
