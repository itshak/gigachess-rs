| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.019x | 0.941x | 1.379x |
| materialize | 1.011x | 0.976x | 1.247x |
| perft_bulk_d4 | 0.926x | 0.903x | 1.249x |
| perft_full_d3 | 0.987x | 0.980x | 1.056x |
| fen_parse | 0.990x | 0.986x | 1.231x |
| fen_write_native_ep | 1.001x | 0.999x | 1.583x |
| snapshot | 0.985x | 0.981x | 0.969x |
| san_parse_all | 1.003x | 0.997x | 1.825x |
| san_render_native | 0.992x | 0.985x | 2.539x |
| apply_children_full | 1.001x | 0.994x | 1.114x |
| make_unmake_full | 0.995x | 0.989x | 1.439x |
| startpos | 1.000x | 1.000x | 1.173x |
| chess960_materialize | 1.046x | 0.994x | 1.389x |
| chess960_perft_d3 | 1.033x | 0.970x | 1.361x |
| polyglot_key | 1.002x | 0.994x | 25.875x |
| polyglot_recompute | 1.005x | 0.998x | 1.079x |
| repetition_key | 0.985x | 0.957x | 19.016x |
| corpus_count | 0.825x | 0.796x | 1.201x |
| corpus_materialize | 0.972x | 0.964x | 1.165x |
| corpus_uci_sorted | 0.992x | 0.992x | 1.450x |
| replay_trusted_native | 1.016x | 1.005x | 0.530x |
| replay_checked_native | 1.000x | 1.000x | 0.982x |

Cold first legal move list: 0.009 ms -> 0.005 ms (1.69x).
