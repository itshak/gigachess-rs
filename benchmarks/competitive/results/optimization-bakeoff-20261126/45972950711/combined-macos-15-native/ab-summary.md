| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.952x | 0.700x | 1.577x |
| materialize | 0.972x | 0.726x | 1.081x |
| perft_bulk_d4 | 1.135x | 0.942x | 1.301x |
| perft_full_d3 | 0.879x | 0.761x | 1.206x |
| fen_parse | 1.002x | 0.896x | 1.137x |
| fen_write_native_ep | 1.105x | 0.793x | 1.806x |
| snapshot | 0.951x | 0.603x | 0.975x |
| san_parse_all | 1.121x | 1.030x | 1.872x |
| san_render_native | 1.030x | 0.904x | 2.882x |
| apply_children_full | 0.954x | 0.845x | 1.100x |
| make_unmake_full | 0.986x | 0.795x | 1.318x |
| startpos | 0.595x | 0.595x | 1.952x |
| chess960_materialize | 1.136x | 1.049x | 1.452x |
| chess960_perft_d3 | 1.142x | 1.110x | 1.703x |
| polyglot_key | 1.079x | 1.069x | 40.476x |
| polyglot_recompute | 0.977x | 0.846x | 1.262x |
| repetition_key | 0.826x | 0.692x | 21.129x |
| corpus_count | 0.996x | 0.934x | 1.131x |
| corpus_materialize | 1.027x | 0.931x | 1.255x |
| corpus_uci_sorted | 1.059x | 1.020x | 1.291x |
| replay_trusted_native | 1.065x | 1.013x | 0.649x |
| replay_checked_native | 1.015x | 0.890x | 1.105x |

Cold first legal move list: 0.008 ms -> 0.011 ms (0.69x).
