| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.977x | 0.972x | 1.517x |
| materialize | 0.993x | 0.988x | 1.316x |
| perft_bulk_d4 | 0.963x | 0.955x | 1.339x |
| perft_full_d3 | 1.033x | 0.991x | 0.859x |
| fen_parse | 1.034x | 1.016x | 1.244x |
| fen_write_native_ep | 0.991x | 0.984x | 1.573x |
| snapshot | 0.992x | 0.988x | 0.956x |
| san_parse_all | 0.925x | 0.868x | 1.708x |
| san_render_native | 0.887x | 0.834x | 2.238x |
| apply_children_full | 1.057x | 1.045x | 1.216x |
| make_unmake_full | 0.938x | 0.923x | 1.453x |
| startpos | 0.927x | 0.927x | 1.221x |
| chess960_materialize | 0.984x | 0.983x | 1.471x |
| chess960_perft_d3 | 0.949x | 0.933x | 1.436x |
| polyglot_key | 0.977x | 0.969x | 25.077x |
| polyglot_recompute | 0.987x | 0.985x | 1.075x |
| repetition_key | 0.957x | 0.941x | 18.122x |
| corpus_count | 0.999x | 0.974x | 1.388x |
| corpus_materialize | 0.999x | 0.989x | 1.234x |
| corpus_uci_sorted | 1.001x | 0.998x | 1.457x |
| replay_trusted_native | 0.967x | 0.951x | 0.593x |
| replay_checked_native | 0.965x | 0.964x | 1.007x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.02x).
