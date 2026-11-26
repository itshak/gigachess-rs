| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.990x | 1.378x |
| materialize | 1.008x | 0.995x | 1.490x |
| perft_bulk_d4 | 1.024x | 1.006x | 1.041x |
| perft_full_d3 | 1.021x | 0.983x | 1.451x |
| fen_parse | 1.068x | 1.000x | 1.131x |
| fen_write_native_ep | 0.973x | 0.927x | 2.008x |
| snapshot | 0.995x | 0.986x | 1.148x |
| san_parse_all | 0.974x | 0.956x | 1.888x |
| san_render_native | 0.993x | 0.976x | 2.132x |
| apply_children_full | 1.016x | 1.005x | 1.327x |
| make_unmake_full | 0.935x | 0.876x | 1.090x |
| startpos | 0.998x | 0.998x | 1.123x |
| chess960_materialize | 1.050x | 1.035x | 1.454x |
| chess960_perft_d3 | 1.019x | 1.000x | 1.400x |
| polyglot_key | 0.963x | 0.923x | 37.984x |
| polyglot_recompute | 1.054x | 1.008x | 1.067x |
| repetition_key | 0.859x | 0.714x | 19.006x |
| corpus_count | 1.011x | 1.005x | 0.893x |
| corpus_materialize | 0.999x | 0.995x | 1.031x |
| corpus_uci_sorted | 1.017x | 1.001x | 1.567x |
| replay_trusted_native | 0.999x | 0.966x | 0.899x |
| replay_checked_native | 0.985x | 0.969x | 1.103x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.89x).
