| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.965x | 0.799x | 1.361x |
| materialize | 0.964x | 0.824x | 1.450x |
| perft_bulk_d4 | 1.000x | 0.992x | 1.212x |
| perft_full_d3 | 1.035x | 1.027x | 1.519x |
| fen_parse | 1.047x | 1.017x | 1.102x |
| fen_write_native_ep | 0.990x | 0.966x | 1.918x |
| snapshot | 1.293x | 1.292x | 1.000x |
| san_parse_all | 0.932x | 0.920x | 1.723x |
| san_render_native | 0.994x | 0.932x | 2.081x |
| apply_children_full | 0.973x | 0.910x | 1.021x |
| make_unmake_full | 0.994x | 0.985x | 1.151x |
| startpos | 0.978x | 0.978x | 0.998x |
| chess960_materialize | 1.045x | 1.040x | 1.432x |
| chess960_perft_d3 | 1.002x | 0.999x | 1.382x |
| polyglot_key | 1.023x | 1.000x | 38.245x |
| polyglot_recompute | 0.998x | 0.996x | 0.989x |
| repetition_key | 1.120x | 0.987x | 20.992x |
| corpus_count | 1.001x | 0.999x | 1.346x |
| corpus_materialize | 1.014x | 1.010x | 1.100x |
| corpus_uci_sorted | 1.000x | 0.999x | 1.554x |
| replay_trusted_native | 1.008x | 1.003x | 0.718x |
| replay_checked_native | 1.012x | 1.012x | 1.044x |

Cold first legal move list: 0.009 ms -> 0.009 ms (1.03x).
