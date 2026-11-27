| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.044x | 1.005x | 1.623x |
| materialize | 0.965x | 0.918x | 1.538x |
| perft_bulk_d4 | 1.063x | 1.033x | 1.266x |
| perft_full_d3 | 1.244x | 1.059x | 1.721x |
| fen_parse | 0.991x | 0.988x | 1.046x |
| fen_write_native_ep | 1.008x | 0.998x | 1.927x |
| snapshot | 0.993x | 0.982x | 1.148x |
| san_parse_all | 1.002x | 0.990x | 1.841x |
| san_render_native | 1.006x | 0.986x | 2.093x |
| apply_children_full | 1.002x | 0.997x | 1.342x |
| make_unmake_full | 0.999x | 0.975x | 1.197x |
| startpos | 1.120x | 1.120x | 1.126x |
| chess960_materialize | 1.014x | 0.999x | 1.532x |
| chess960_perft_d3 | 1.007x | 0.978x | 1.489x |
| polyglot_key | 1.000x | 1.000x | 39.561x |
| polyglot_recompute | 1.019x | 1.018x | 1.020x |
| repetition_key | 0.942x | 0.884x | 22.416x |
| corpus_count | 1.140x | 1.112x | 1.291x |
| corpus_materialize | 1.008x | 0.999x | 1.074x |
| corpus_uci_sorted | 1.014x | 1.008x | 1.576x |
| replay_trusted_native | 1.019x | 1.000x | 0.930x |
| replay_checked_native | 1.010x | 0.999x | 1.196x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.93x).
