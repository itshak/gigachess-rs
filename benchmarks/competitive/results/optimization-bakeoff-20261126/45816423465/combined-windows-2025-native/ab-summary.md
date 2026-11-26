| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.898x | 1.367x |
| materialize | 1.008x | 0.947x | 1.105x |
| perft_bulk_d4 | 0.982x | 0.945x | 1.306x |
| perft_full_d3 | 1.006x | 0.939x | 1.054x |
| fen_parse | 1.017x | 1.001x | 1.447x |
| fen_write_native_ep | 1.020x | 0.990x | 1.598x |
| snapshot | 1.517x | 1.503x | 1.169x |
| san_parse_all | 1.063x | 1.028x | 1.970x |
| san_render_native | 0.972x | 0.951x | 3.329x |
| apply_children_full | 1.081x | 1.052x | 1.109x |
| make_unmake_full | 1.048x | 1.011x | 1.317x |
| startpos | 1.194x | 1.194x | 1.196x |
| chess960_materialize | 1.064x | 1.013x | 1.273x |
| chess960_perft_d3 | 1.086x | 0.992x | 1.535x |
| polyglot_key | 1.031x | 0.995x | 20.480x |
| polyglot_recompute | 0.990x | 0.985x | 0.988x |
| repetition_key | 0.899x | 0.808x | 15.674x |
| corpus_count | 0.900x | 0.807x | 1.199x |
| corpus_materialize | 1.032x | 0.948x | 1.134x |
| corpus_uci_sorted | 1.011x | 0.985x | 1.499x |
| replay_trusted_native | 1.031x | 0.999x | 0.812x |
| replay_checked_native | 1.023x | 1.017x | 0.941x |

Cold first legal move list: 0.023 ms -> 0.019 ms (1.19x).
