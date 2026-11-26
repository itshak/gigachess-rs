| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.979x | 0.852x | 1.487x |
| materialize | 1.009x | 0.885x | 1.256x |
| perft_bulk_d4 | 0.950x | 0.917x | 1.278x |
| perft_full_d3 | 0.995x | 0.952x | 1.113x |
| fen_parse | 1.136x | 1.100x | 1.290x |
| fen_write_native_ep | 1.034x | 1.009x | 1.551x |
| snapshot | 0.966x | 0.831x | 0.976x |
| san_parse_all | 0.999x | 0.993x | 2.103x |
| san_render_native | 0.980x | 0.914x | 3.640x |
| apply_children_full | 0.903x | 0.864x | 1.237x |
| make_unmake_full | 0.889x | 0.829x | 1.255x |
| startpos | 0.839x | 0.839x | 1.054x |
| chess960_materialize | 1.076x | 0.993x | 1.440x |
| chess960_perft_d3 | 0.942x | 0.928x | 1.489x |
| polyglot_key | 1.007x | 0.972x | 26.795x |
| polyglot_recompute | 1.019x | 1.017x | 1.047x |
| repetition_key | 0.980x | 0.970x | 20.094x |
| corpus_count | 1.023x | 1.012x | 1.393x |
| corpus_materialize | 1.031x | 1.029x | 1.198x |
| corpus_uci_sorted | 0.982x | 0.972x | 1.409x |
| replay_trusted_native | 0.961x | 0.857x | 0.801x |
| replay_checked_native | 0.961x | 0.952x | 0.958x |

Cold first legal move list: 0.017 ms -> 0.015 ms (1.13x).
