| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.055x | 1.011x | 1.372x |
| materialize | 0.943x | 0.891x | 1.647x |
| perft_bulk_d4 | 1.081x | 1.051x | 1.286x |
| perft_full_d3 | 1.154x | 1.082x | 1.554x |
| fen_parse | 1.013x | 0.953x | 1.221x |
| fen_write_native_ep | 1.013x | 0.905x | 1.886x |
| snapshot | 1.018x | 0.984x | 1.006x |
| san_parse_all | 1.016x | 0.963x | 1.939x |
| san_render_native | 0.994x | 0.951x | 2.320x |
| apply_children_full | 1.015x | 0.956x | 1.391x |
| make_unmake_full | 1.016x | 0.929x | 1.293x |
| startpos | 0.977x | 0.977x | 1.446x |
| chess960_materialize | 0.961x | 0.958x | 1.590x |
| chess960_perft_d3 | 0.986x | 0.979x | 1.522x |
| polyglot_key | 0.939x | 0.937x | 41.097x |
| polyglot_recompute | 1.029x | 1.017x | 1.027x |
| repetition_key | 1.011x | 0.922x | 25.655x |
| corpus_count | 1.116x | 1.114x | 1.351x |
| corpus_materialize | 1.015x | 1.011x | 1.207x |
| corpus_uci_sorted | 1.024x | 1.011x | 1.528x |
| replay_trusted_native | 0.985x | 0.962x | 0.894x |
| replay_checked_native | 1.019x | 1.000x | 1.171x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.85x).
