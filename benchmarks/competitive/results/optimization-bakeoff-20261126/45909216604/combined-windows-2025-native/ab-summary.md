| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.909x | 0.766x | 1.436x |
| materialize | 0.918x | 0.727x | 1.145x |
| perft_bulk_d4 | 0.994x | 0.885x | 1.396x |
| perft_full_d3 | 0.960x | 0.851x | 1.316x |
| fen_parse | 0.889x | 0.830x | 1.116x |
| fen_write_native_ep | 0.946x | 0.874x | 1.560x |
| snapshot | 0.962x | 0.846x | 0.968x |
| san_parse_all | 0.971x | 0.896x | 2.136x |
| san_render_native | 0.959x | 0.821x | 3.719x |
| apply_children_full | 0.976x | 0.932x | 1.397x |
| make_unmake_full | 0.977x | 0.894x | 1.377x |
| startpos | 1.002x | 1.002x | 1.250x |
| chess960_materialize | 1.045x | 1.008x | 1.335x |
| chess960_perft_d3 | 1.000x | 0.985x | 1.521x |
| polyglot_key | 0.997x | 0.939x | 28.038x |
| polyglot_recompute | 0.995x | 0.988x | 1.034x |
| repetition_key | 0.968x | 0.921x | 19.850x |
| corpus_count | 1.043x | 0.997x | 1.431x |
| corpus_materialize | 1.009x | 1.009x | 1.133x |
| corpus_uci_sorted | 0.990x | 0.989x | 1.396x |
| replay_trusted_native | 1.034x | 1.006x | 0.838x |
| replay_checked_native | 1.013x | 1.000x | 1.113x |

Cold first legal move list: 0.020 ms -> 0.018 ms (1.10x).
