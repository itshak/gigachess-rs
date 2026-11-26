| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.022x | 0.978x | 1.596x |
| materialize | 1.075x | 1.038x | 1.603x |
| perft_bulk_d4 | 1.012x | 0.990x | 1.208x |
| perft_full_d3 | 1.008x | 1.004x | 1.348x |
| fen_parse | 0.938x | 0.838x | 1.070x |
| fen_write_native_ep | 0.983x | 0.948x | 1.971x |
| snapshot | 0.995x | 0.987x | 1.146x |
| san_parse_all | 1.022x | 0.987x | 1.878x |
| san_render_native | 1.011x | 0.992x | 2.104x |
| apply_children_full | 0.999x | 0.995x | 1.336x |
| make_unmake_full | 1.003x | 0.992x | 1.200x |
| startpos | 1.119x | 1.119x | 1.121x |
| chess960_materialize | 1.011x | 1.003x | 1.545x |
| chess960_perft_d3 | 1.004x | 1.004x | 1.488x |
| polyglot_key | 0.972x | 0.947x | 38.532x |
| polyglot_recompute | 0.985x | 0.981x | 0.982x |
| repetition_key | 0.942x | 0.911x | 22.214x |
| corpus_count | 0.951x | 0.899x | 1.070x |
| corpus_materialize | 1.018x | 1.011x | 1.087x |
| corpus_uci_sorted | 1.027x | 1.024x | 1.569x |
| replay_trusted_native | 1.025x | 1.007x | 0.907x |
| replay_checked_native | 1.000x | 0.998x | 1.157x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.08x).
