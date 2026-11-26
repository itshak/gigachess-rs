| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.981x | 0.967x | 1.255x |
| materialize | 0.917x | 0.874x | 1.629x |
| perft_bulk_d4 | 0.955x | 0.934x | 1.128x |
| perft_full_d3 | 0.933x | 0.883x | 1.181x |
| fen_parse | 0.920x | 0.867x | 1.078x |
| fen_write_native_ep | 0.996x | 0.981x | 1.797x |
| snapshot | 0.996x | 0.980x | 0.994x |
| san_parse_all | 1.034x | 0.999x | 1.850x |
| san_render_native | 0.906x | 0.881x | 2.033x |
| apply_children_full | 0.978x | 0.965x | 1.246x |
| make_unmake_full | 0.890x | 0.852x | 1.345x |
| startpos | 0.999x | 0.999x | 1.294x |
| chess960_materialize | 0.911x | 0.847x | 1.408x |
| chess960_perft_d3 | 0.921x | 0.886x | 1.382x |
| polyglot_key | 0.988x | 0.978x | 40.943x |
| polyglot_recompute | 0.930x | 0.895x | 0.950x |
| repetition_key | 0.961x | 0.956x | 19.897x |
| corpus_count | 0.975x | 0.975x | 1.103x |
| corpus_materialize | 0.992x | 0.991x | 1.123x |
| corpus_uci_sorted | 0.980x | 0.977x | 1.529x |
| replay_trusted_native | 0.983x | 0.964x | 0.857x |
| replay_checked_native | 0.961x | 0.944x | 1.074x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.84x).
