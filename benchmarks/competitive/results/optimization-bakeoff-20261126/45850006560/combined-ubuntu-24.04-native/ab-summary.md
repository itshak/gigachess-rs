| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.973x | 0.951x | 1.441x |
| materialize | 0.946x | 0.925x | 1.140x |
| perft_bulk_d4 | 0.975x | 0.946x | 1.307x |
| perft_full_d3 | 0.839x | 0.808x | 1.001x |
| fen_parse | 0.826x | 0.767x | 0.966x |
| fen_write_native_ep | 1.004x | 0.997x | 1.852x |
| snapshot | 0.730x | 0.728x | 1.000x |
| san_parse_all | 0.881x | 0.859x | 1.932x |
| san_render_native | 0.951x | 0.868x | 1.993x |
| apply_children_full | 0.925x | 0.857x | 1.220x |
| make_unmake_full | 0.966x | 0.935x | 1.165x |
| startpos | 0.754x | 0.754x | 0.978x |
| chess960_materialize | 0.894x | 0.846x | 1.462x |
| chess960_perft_d3 | 0.935x | 0.891x | 1.490x |
| polyglot_key | 1.028x | 1.003x | 20.990x |
| polyglot_recompute | 0.971x | 0.962x | 0.982x |
| repetition_key | 1.019x | 0.957x | 19.201x |
| corpus_count | 0.991x | 0.990x | 1.272x |
| corpus_materialize | 0.970x | 0.956x | 1.242x |
| corpus_uci_sorted | 1.015x | 1.014x | 1.657x |
| replay_trusted_native | 0.864x | 0.853x | 0.971x |
| replay_checked_native | 0.925x | 0.922x | 0.938x |

Cold first legal move list: 0.010 ms -> 0.006 ms (1.51x).
