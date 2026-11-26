| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.028x | 0.977x | 1.226x |
| materialize | 0.985x | 0.951x | 1.649x |
| perft_bulk_d4 | 0.925x | 0.883x | 1.074x |
| perft_full_d3 | 0.959x | 0.884x | 1.456x |
| fen_parse | 1.011x | 0.986x | 1.184x |
| fen_write_native_ep | 1.006x | 0.992x | 1.763x |
| snapshot | 1.120x | 1.115x | 1.001x |
| san_parse_all | 0.992x | 0.955x | 2.031x |
| san_render_native | 0.823x | 0.790x | 1.803x |
| apply_children_full | 0.986x | 0.961x | 1.216x |
| make_unmake_full | 1.003x | 0.980x | 1.173x |
| startpos | 0.999x | 0.999x | 1.355x |
| chess960_materialize | 1.018x | 0.960x | 1.321x |
| chess960_perft_d3 | 1.046x | 0.985x | 1.392x |
| polyglot_key | 1.075x | 0.999x | 42.926x |
| polyglot_recompute | 1.038x | 1.028x | 1.068x |
| repetition_key | 0.766x | 0.716x | 18.283x |
| corpus_count | 0.815x | 0.799x | 1.016x |
| corpus_materialize | 0.933x | 0.914x | 1.083x |
| corpus_uci_sorted | 0.984x | 0.981x | 1.511x |
| replay_trusted_native | 1.013x | 0.988x | 0.843x |
| replay_checked_native | 1.003x | 0.972x | 1.104x |

Cold first legal move list: 0.010 ms -> 0.005 ms (1.87x).
