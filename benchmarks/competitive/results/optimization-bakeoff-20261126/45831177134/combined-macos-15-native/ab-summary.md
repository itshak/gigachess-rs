| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.962x | 0.675x | 1.193x |
| materialize | 0.991x | 0.847x | 1.119x |
| perft_bulk_d4 | 0.903x | 0.734x | 1.034x |
| perft_full_d3 | 0.979x | 0.836x | 1.366x |
| fen_parse | 0.938x | 0.710x | 1.179x |
| fen_write_native_ep | 0.966x | 0.840x | 1.801x |
| snapshot | 1.141x | 0.594x | 0.907x |
| san_parse_all | 1.063x | 0.944x | 1.743x |
| san_render_native | 1.139x | 0.868x | 2.935x |
| apply_children_full | 1.070x | 0.865x | 1.096x |
| make_unmake_full | 1.021x | 0.881x | 1.285x |
| startpos | 0.866x | 0.866x | 1.711x |
| chess960_materialize | 0.888x | 0.854x | 1.246x |
| chess960_perft_d3 | 1.217x | 1.213x | 1.304x |
| polyglot_key | 1.065x | 0.951x | 39.790x |
| polyglot_recompute | 1.132x | 1.018x | 0.901x |
| repetition_key | 1.176x | 1.116x | 22.331x |
| corpus_count | 0.951x | 0.860x | 0.909x |
| corpus_materialize | 1.133x | 1.094x | 1.186x |
| corpus_uci_sorted | 1.021x | 0.907x | 1.445x |
| replay_trusted_native | 0.952x | 0.909x | 0.582x |
| replay_checked_native | 1.054x | 1.025x | 1.110x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.85x).
