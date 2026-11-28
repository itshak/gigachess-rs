| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.033x | 0.984x | 1.570x |
| materialize | 0.994x | 0.931x | 1.176x |
| perft_bulk_d4 | 1.037x | 0.941x | 1.320x |
| perft_full_d3 | 1.196x | 0.851x | 1.493x |
| fen_parse | 0.917x | 0.891x | 1.155x |
| fen_write_native_ep | 0.951x | 0.846x | 1.480x |
| snapshot | 0.990x | 0.967x | 1.493x |
| san_parse_all | 0.997x | 0.973x | 1.740x |
| san_render_native | 1.005x | 0.965x | 2.346x |
| apply_children_full | 1.009x | 0.935x | 1.367x |
| make_unmake_full | 0.999x | 0.957x | 1.216x |
| startpos | 1.001x | 1.001x | 1.250x |
| chess960_materialize | 1.003x | 1.000x | 1.507x |
| chess960_perft_d3 | 1.019x | 1.014x | 1.536x |
| polyglot_key | 1.000x | 0.986x | 35.723x |
| polyglot_recompute | 1.633x | 1.401x | 1.566x |
| repetition_key | 1.018x | 1.007x | 27.036x |
| corpus_count | 1.088x | 1.059x | 1.235x |
| corpus_materialize | 1.036x | 1.013x | 1.155x |
| corpus_uci_sorted | 0.991x | 0.989x | 1.660x |
| replay_trusted_native | 1.034x | 1.028x | 0.778x |
| replay_checked_native | 0.998x | 0.990x | 1.188x |

Cold first legal move list: 0.008 ms -> 0.009 ms (0.89x).
