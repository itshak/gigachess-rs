| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 0.977x | 1.311x |
| materialize | 1.032x | 0.977x | 1.147x |
| perft_bulk_d4 | 1.025x | 0.973x | 1.250x |
| perft_full_d3 | 0.976x | 0.914x | 1.218x |
| fen_parse | 1.074x | 0.941x | 1.381x |
| fen_write_native_ep | 0.961x | 0.942x | 1.434x |
| snapshot | 0.970x | 0.808x | 1.445x |
| san_parse_all | 1.001x | 0.959x | 1.969x |
| san_render_native | 1.042x | 0.996x | 3.816x |
| apply_children_full | 0.717x | 0.654x | 0.955x |
| make_unmake_full | 1.002x | 0.974x | 1.286x |
| startpos | 0.965x | 0.965x | 1.004x |
| chess960_materialize | 0.981x | 0.957x | 1.280x |
| chess960_perft_d3 | 0.994x | 0.945x | 1.372x |
| polyglot_key | 1.072x | 1.055x | 36.832x |
| polyglot_recompute | 0.979x | 0.971x | 0.996x |
| repetition_key | 1.189x | 1.129x | 29.016x |
| corpus_count | 1.016x | 1.011x | 1.090x |
| corpus_materialize | 1.028x | 0.997x | 0.993x |
| corpus_uci_sorted | 1.131x | 0.945x | 1.409x |
| replay_trusted_native | 0.905x | 0.879x | 0.671x |
| replay_checked_native | 0.976x | 0.890x | 1.079x |

Cold first legal move list: 0.012 ms -> 0.012 ms (1.01x).
