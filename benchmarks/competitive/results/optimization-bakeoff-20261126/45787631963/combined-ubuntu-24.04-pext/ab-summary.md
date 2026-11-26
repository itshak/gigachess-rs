| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.960x | 0.921x | 1.412x |
| materialize | 1.021x | 0.957x | 1.498x |
| perft_bulk_d4 | 0.829x | 0.720x | 1.053x |
| perft_full_d3 | 0.939x | 0.873x | 1.420x |
| fen_parse | 1.025x | 1.013x | 1.142x |
| fen_write_native_ep | 1.008x | 0.980x | 1.900x |
| snapshot | 0.500x | 0.499x | 1.000x |
| san_parse_all | 1.145x | 1.116x | 1.914x |
| san_render_native | 1.017x | 1.004x | 2.222x |
| apply_children_full | 0.983x | 0.950x | 1.265x |
| make_unmake_full | 0.996x | 0.986x | 1.120x |
| startpos | 0.303x | 0.303x | 0.607x |
| chess960_materialize | 1.006x | 1.000x | 1.473x |
| chess960_perft_d3 | 0.978x | 0.974x | 1.349x |
| polyglot_key | 1.031x | 1.000x | 39.712x |
| polyglot_recompute | 1.003x | 1.002x | 1.021x |
| repetition_key | 1.024x | 1.008x | 19.289x |
| corpus_count | 0.681x | 0.617x | 0.952x |
| corpus_materialize | 0.955x | 0.946x | 1.061x |
| corpus_uci_sorted | 0.989x | 0.981x | 1.584x |
| replay_trusted_native | 0.999x | 0.991x | 0.734x |
| replay_checked_native | 1.009x | 1.006x | 1.074x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.26x).
