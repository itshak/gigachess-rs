| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.012x | 0.983x | 1.571x |
| materialize | 1.011x | 1.000x | 1.358x |
| perft_bulk_d4 | 1.035x | 1.015x | 1.442x |
| perft_full_d3 | 1.133x | 1.003x | 1.514x |
| fen_parse | 1.017x | 0.938x | 1.250x |
| fen_write_native_ep | 1.001x | 0.998x | 1.577x |
| snapshot | 0.999x | 0.997x | 0.943x |
| san_parse_all | 1.001x | 0.997x | 1.817x |
| san_render_native | 1.007x | 0.994x | 2.687x |
| apply_children_full | 1.004x | 0.990x | 1.158x |
| make_unmake_full | 1.009x | 1.001x | 1.567x |
| startpos | 1.000x | 1.000x | 1.292x |
| chess960_materialize | 1.008x | 1.008x | 1.506x |
| chess960_perft_d3 | 1.014x | 1.007x | 1.509x |
| polyglot_key | 1.023x | 1.001x | 25.612x |
| polyglot_recompute | 1.368x | 1.280x | 1.476x |
| repetition_key | 1.025x | 0.999x | 19.346x |
| corpus_count | 1.093x | 1.051x | 1.493x |
| corpus_materialize | 0.995x | 0.995x | 1.240x |
| corpus_uci_sorted | 0.994x | 0.993x | 1.452x |
| replay_trusted_native | 1.099x | 1.081x | 0.691x |
| replay_checked_native | 1.019x | 1.019x | 1.148x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.84x).
