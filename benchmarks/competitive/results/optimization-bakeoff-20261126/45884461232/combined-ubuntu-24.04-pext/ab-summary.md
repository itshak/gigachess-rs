| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.991x | 1.566x |
| materialize | 1.006x | 1.000x | 1.608x |
| perft_bulk_d4 | 1.003x | 0.985x | 1.207x |
| perft_full_d3 | 1.000x | 0.996x | 1.358x |
| fen_parse | 0.939x | 0.833x | 1.104x |
| fen_write_native_ep | 1.012x | 1.004x | 2.043x |
| snapshot | 1.001x | 0.999x | 1.154x |
| san_parse_all | 1.023x | 0.985x | 1.880x |
| san_render_native | 1.000x | 0.999x | 2.201x |
| apply_children_full | 0.806x | 0.739x | 1.082x |
| make_unmake_full | 0.851x | 0.774x | 1.023x |
| startpos | 1.002x | 1.002x | 1.120x |
| chess960_materialize | 1.011x | 0.993x | 1.526x |
| chess960_perft_d3 | 1.000x | 0.996x | 1.482x |
| polyglot_key | 1.083x | 1.001x | 39.908x |
| polyglot_recompute | 0.976x | 0.973x | 0.968x |
| repetition_key | 1.060x | 0.999x | 23.257x |
| corpus_count | 0.956x | 0.912x | 1.072x |
| corpus_materialize | 1.011x | 1.008x | 1.096x |
| corpus_uci_sorted | 1.013x | 1.008x | 1.570x |
| replay_trusted_native | 0.829x | 0.732x | 0.743x |
| replay_checked_native | 0.950x | 0.937x | 1.130x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.96x).
