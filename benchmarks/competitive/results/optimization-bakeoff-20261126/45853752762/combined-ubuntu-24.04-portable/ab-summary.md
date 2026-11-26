| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.997x | 1.299x |
| materialize | 1.096x | 1.044x | 1.720x |
| perft_bulk_d4 | 1.000x | 0.993x | 1.190x |
| perft_full_d3 | 0.988x | 0.973x | 1.323x |
| fen_parse | 1.014x | 0.987x | 1.240x |
| fen_write_native_ep | 0.980x | 0.968x | 1.917x |
| snapshot | 0.999x | 0.984x | 1.002x |
| san_parse_all | 0.945x | 0.932x | 1.774x |
| san_render_native | 0.963x | 0.939x | 2.120x |
| apply_children_full | 0.941x | 0.909x | 1.234x |
| make_unmake_full | 0.893x | 0.850x | 1.137x |
| startpos | 1.001x | 1.001x | 1.440x |
| chess960_materialize | 1.047x | 1.037x | 1.598x |
| chess960_perft_d3 | 0.986x | 0.973x | 1.484x |
| polyglot_key | 0.997x | 0.994x | 43.976x |
| polyglot_recompute | 1.014x | 1.010x | 1.065x |
| repetition_key | 0.986x | 0.985x | 21.606x |
| corpus_count | 0.966x | 0.935x | 1.131x |
| corpus_materialize | 1.004x | 1.001x | 1.152x |
| corpus_uci_sorted | 1.001x | 0.999x | 1.566x |
| replay_trusted_native | 0.979x | 0.975x | 0.898x |
| replay_checked_native | 1.028x | 1.017x | 1.089x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.09x).
