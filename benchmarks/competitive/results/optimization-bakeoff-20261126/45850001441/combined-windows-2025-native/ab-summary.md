| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 0.973x | 1.555x |
| materialize | 1.004x | 0.985x | 1.390x |
| perft_bulk_d4 | 0.994x | 0.961x | 1.185x |
| perft_full_d3 | 1.238x | 1.052x | 1.787x |
| fen_parse | 0.974x | 0.804x | 1.232x |
| fen_write_native_ep | 1.000x | 0.865x | 1.617x |
| snapshot | 0.843x | 0.694x | 1.089x |
| san_parse_all | 1.019x | 0.983x | 1.965x |
| san_render_native | 1.005x | 0.967x | 4.076x |
| apply_children_full | 0.995x | 0.966x | 1.318x |
| make_unmake_full | 1.012x | 0.976x | 1.212x |
| startpos | 0.996x | 0.996x | 0.991x |
| chess960_materialize | 1.050x | 1.012x | 1.389x |
| chess960_perft_d3 | 0.999x | 0.999x | 1.512x |
| polyglot_key | 0.999x | 0.999x | 40.119x |
| polyglot_recompute | 1.045x | 1.039x | 0.969x |
| repetition_key | 1.008x | 1.006x | 28.231x |
| corpus_count | 1.116x | 1.044x | 0.980x |
| corpus_materialize | 1.012x | 1.012x | 1.072x |
| corpus_uci_sorted | 1.057x | 1.034x | 1.506x |
| replay_trusted_native | 1.061x | 1.051x | 0.852x |
| replay_checked_native | 1.261x | 1.116x | 1.059x |

Cold first legal move list: 0.017 ms -> 0.019 ms (0.90x).
