| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.206x | 1.029x | 1.426x |
| materialize | 0.998x | 0.703x | 0.992x |
| perft_bulk_d4 | 1.041x | 0.860x | 1.287x |
| perft_full_d3 | 0.929x | 0.785x | 1.315x |
| fen_parse | 0.971x | 0.862x | 1.219x |
| fen_write_native_ep | 0.941x | 0.755x | 1.129x |
| snapshot | 1.049x | 0.769x | 0.886x |
| san_parse_all | 0.901x | 0.773x | 1.936x |
| san_render_native | 0.992x | 0.854x | 2.824x |
| apply_children_full | 0.926x | 0.794x | 0.961x |
| make_unmake_full | 1.070x | 0.958x | 1.173x |
| startpos | 2.119x | 2.119x | 1.950x |
| chess960_materialize | 1.048x | 0.885x | 1.360x |
| chess960_perft_d3 | 0.875x | 0.797x | 1.090x |
| polyglot_key | 1.158x | 1.084x | 46.070x |
| polyglot_recompute | 0.916x | 0.881x | 0.968x |
| repetition_key | 0.996x | 0.977x | 20.724x |
| corpus_count | 0.932x | 0.931x | 1.312x |
| corpus_materialize | 1.020x | 0.969x | 1.142x |
| corpus_uci_sorted | 0.980x | 0.913x | 1.223x |
| replay_trusted_native | 1.011x | 0.964x | 0.552x |
| replay_checked_native | 1.276x | 1.004x | 1.047x |

Cold first legal move list: 0.015 ms -> 0.023 ms (0.65x).
