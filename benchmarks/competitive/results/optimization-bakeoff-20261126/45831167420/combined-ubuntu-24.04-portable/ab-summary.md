| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.074x | 0.919x | 1.289x |
| materialize | 1.076x | 0.983x | 1.342x |
| perft_bulk_d4 | 1.096x | 1.002x | 1.294x |
| perft_full_d3 | 1.002x | 0.936x | 1.340x |
| fen_parse | 1.217x | 1.163x | 1.339x |
| fen_write_native_ep | 1.001x | 0.997x | 1.701x |
| snapshot | 1.000x | 0.998x | 1.009x |
| san_parse_all | 1.001x | 0.994x | 2.018x |
| san_render_native | 0.997x | 0.994x | 2.308x |
| apply_children_full | 0.999x | 0.990x | 1.372x |
| make_unmake_full | 1.001x | 0.993x | 1.410x |
| startpos | 1.000x | 1.000x | 1.321x |
| chess960_materialize | 1.158x | 1.101x | 1.479x |
| chess960_perft_d3 | 1.063x | 1.050x | 1.545x |
| polyglot_key | 0.958x | 0.922x | 26.299x |
| polyglot_recompute | 1.022x | 1.019x | 1.033x |
| repetition_key | 1.063x | 1.033x | 20.543x |
| corpus_count | 1.081x | 1.053x | 1.267x |
| corpus_materialize | 1.077x | 1.054x | 1.184x |
| corpus_uci_sorted | 1.021x | 1.019x | 1.792x |
| replay_trusted_native | 1.003x | 1.003x | 0.866x |
| replay_checked_native | 1.008x | 0.994x | 1.048x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.04x).
