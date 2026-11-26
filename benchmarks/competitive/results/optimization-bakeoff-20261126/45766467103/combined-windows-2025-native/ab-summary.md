| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.942x | 0.916x | 1.216x |
| materialize | 1.009x | 0.972x | 1.080x |
| perft_bulk_d4 | 1.148x | 1.121x | 1.209x |
| perft_full_d3 | 0.955x | 0.868x | 0.604x |
| fen_parse | 1.063x | 0.980x | 1.388x |
| fen_write_native_ep | 1.017x | 0.991x | 1.219x |
| snapshot | 0.984x | 0.960x | 0.843x |
| san_parse_all | 1.426x | 1.369x | 1.214x |
| san_render_native | 1.522x | 1.484x | 2.676x |
| apply_children_full | 1.110x | 1.090x | 0.753x |
| make_unmake_full | 1.071x | 1.039x | 0.906x |
| startpos | 1.143x | 1.143x | 1.150x |
| chess960_materialize | 1.155x | 1.025x | 1.193x |
| chess960_perft_d3 | 1.234x | 1.082x | 1.206x |
| polyglot_key | 1.020x | 1.018x | 23.325x |
| polyglot_recompute | 1.033x | 1.010x | 1.023x |
| repetition_key | 1.123x | 1.105x | 11.411x |
| corpus_count | 0.972x | 0.971x | 1.236x |
| corpus_materialize | 1.054x | 1.051x | 1.161x |
| corpus_uci_sorted | 1.197x | 1.190x | 0.833x |
| replay_trusted_native | 1.122x | 1.040x | 0.543x |
| replay_checked_native | 1.225x | 1.110x | 0.816x |
