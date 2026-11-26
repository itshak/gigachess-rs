| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 1.002x | 1.198x |
| materialize | 1.023x | 1.006x | 1.141x |
| perft_bulk_d4 | 1.048x | 1.033x | 0.979x |
| perft_full_d3 | 1.057x | 1.022x | 0.687x |
| fen_parse | 1.137x | 1.112x | 1.084x |
| fen_write_native_ep | 1.003x | 0.989x | 0.927x |
| snapshot | 0.924x | 0.922x | 0.852x |
| san_parse_all | 1.093x | 1.054x | 0.988x |
| san_render_native | 1.057x | 1.043x | 1.258x |
| apply_children_full | 1.057x | 1.031x | 0.818x |
| make_unmake_full | 1.088x | 1.066x | 0.903x |
| startpos | 0.968x | 0.968x | 0.727x |
| chess960_materialize | 1.047x | 0.982x | 1.254x |
| chess960_perft_d3 | 1.066x | 1.025x | 1.019x |
| polyglot_key | 0.980x | 0.956x | 25.649x |
| polyglot_recompute | 1.023x | 1.022x | 1.059x |
| repetition_key | 1.042x | 1.005x | 14.195x |
| corpus_count | 1.002x | 1.002x | 1.210x |
| corpus_materialize | 0.992x | 0.988x | 1.158x |
| corpus_uci_sorted | 0.977x | 0.976x | 0.648x |
| replay_trusted_native | 1.083x | 1.032x | 0.527x |
| replay_checked_native | 1.095x | 1.052x | 0.766x |

Cold first legal move list: 275.724 ms → 268.687 ms (1.03x).
