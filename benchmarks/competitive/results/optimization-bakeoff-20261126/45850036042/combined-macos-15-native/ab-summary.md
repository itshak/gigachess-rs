| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.967x | 0.687x | 1.523x |
| materialize | 1.038x | 0.758x | 1.053x |
| perft_bulk_d4 | 1.093x | 0.913x | 1.280x |
| perft_full_d3 | 1.088x | 0.889x | 1.322x |
| fen_parse | 1.084x | 0.873x | 1.152x |
| fen_write_native_ep | 1.068x | 0.952x | 1.786x |
| snapshot | 1.157x | 0.762x | 0.884x |
| san_parse_all | 1.034x | 0.842x | 1.812x |
| san_render_native | 1.087x | 0.750x | 2.664x |
| apply_children_full | 0.988x | 0.812x | 1.007x |
| make_unmake_full | 1.036x | 0.783x | 1.227x |
| startpos | 0.819x | 0.819x | 0.766x |
| chess960_materialize | 0.934x | 0.834x | 1.485x |
| chess960_perft_d3 | 1.092x | 1.073x | 1.676x |
| polyglot_key | 1.210x | 1.177x | 39.213x |
| polyglot_recompute | 1.156x | 1.054x | 0.999x |
| repetition_key | 1.133x | 1.127x | 21.449x |
| corpus_count | 1.136x | 0.998x | 1.263x |
| corpus_materialize | 1.058x | 0.986x | 1.241x |
| corpus_uci_sorted | 0.903x | 0.864x | 1.180x |
| replay_trusted_native | 0.965x | 0.894x | 0.734x |
| replay_checked_native | 1.100x | 0.936x | 1.125x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.11x).
