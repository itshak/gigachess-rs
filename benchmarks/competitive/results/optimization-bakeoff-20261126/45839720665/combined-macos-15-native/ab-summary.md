| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.841x | 0.672x | 1.419x |
| materialize | 0.927x | 0.726x | 1.049x |
| perft_bulk_d4 | 1.078x | 0.890x | 1.260x |
| perft_full_d3 | 1.118x | 0.866x | 1.169x |
| fen_parse | 0.993x | 0.735x | 1.210x |
| fen_write_native_ep | 1.039x | 0.830x | 1.773x |
| snapshot | 0.854x | 0.559x | 0.909x |
| san_parse_all | 0.933x | 0.774x | 1.748x |
| san_render_native | 0.997x | 0.854x | 2.582x |
| apply_children_full | 0.985x | 0.817x | 1.063x |
| make_unmake_full | 0.935x | 0.694x | 1.281x |
| startpos | 1.016x | 1.016x | 1.201x |
| chess960_materialize | 1.007x | 0.997x | 1.539x |
| chess960_perft_d3 | 1.163x | 1.115x | 1.529x |
| polyglot_key | 0.960x | 0.870x | 35.691x |
| polyglot_recompute | 1.026x | 0.981x | 0.957x |
| repetition_key | 0.876x | 0.843x | 18.286x |
| corpus_count | 0.921x | 0.921x | 1.301x |
| corpus_materialize | 1.258x | 1.196x | 1.171x |
| corpus_uci_sorted | 1.017x | 0.939x | 1.339x |
| replay_trusted_native | 1.280x | 1.141x | 0.697x |
| replay_checked_native | 1.255x | 1.165x | 1.169x |

Cold first legal move list: 0.007 ms -> 0.012 ms (0.59x).
