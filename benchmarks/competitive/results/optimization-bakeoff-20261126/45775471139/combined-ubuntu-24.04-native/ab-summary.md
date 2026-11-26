| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.965x | 0.917x | 1.216x |
| materialize | 0.990x | 0.952x | 1.122x |
| perft_bulk_d4 | 0.969x | 0.946x | 1.204x |
| perft_full_d3 | 0.896x | 0.784x | 0.698x |
| fen_parse | 0.911x | 0.872x | 1.092x |
| fen_write_native_ep | 0.996x | 0.986x | 0.937x |
| snapshot | 0.730x | 0.728x | 1.001x |
| san_parse_all | 0.930x | 0.879x | 1.781x |
| san_render_native | 0.961x | 0.930x | 2.176x |
| apply_children_full | 0.905x | 0.881x | 0.976x |
| make_unmake_full | 0.951x | 0.932x | 0.973x |
| startpos | 0.927x | 0.927x | 1.200x |
| chess960_materialize | 0.947x | 0.932x | 1.206x |
| chess960_perft_d3 | 0.974x | 0.951x | 1.238x |
| polyglot_key | 1.095x | 1.064x | 30.302x |
| polyglot_recompute | 0.976x | 0.974x | 1.010x |
| repetition_key | 1.086x | 1.040x | 22.978x |
| corpus_count | 0.980x | 0.972x | 1.285x |
| corpus_materialize | 0.976x | 0.971x | 1.141x |
| corpus_uci_sorted | 1.014x | 1.012x | 0.988x |
| replay_trusted_native | 0.918x | 0.911x | 0.581x |
| replay_checked_native | 0.974x | 0.974x | 0.892x |

Cold first legal move list: 0.013 ms -> 0.013 ms (0.99x).
