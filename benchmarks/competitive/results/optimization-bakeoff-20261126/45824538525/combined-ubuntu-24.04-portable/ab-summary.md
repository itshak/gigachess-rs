| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.993x | 1.199x |
| materialize | 0.942x | 0.885x | 1.254x |
| perft_bulk_d4 | 1.010x | 0.983x | 1.182x |
| perft_full_d3 | 0.998x | 0.977x | 1.129x |
| fen_parse | 1.059x | 0.956x | 1.367x |
| fen_write_native_ep | 0.984x | 0.929x | 1.655x |
| snapshot | 0.999x | 0.992x | 0.997x |
| san_parse_all | 0.992x | 0.952x | 1.950x |
| san_render_native | 0.995x | 0.960x | 2.283x |
| apply_children_full | 0.992x | 0.896x | 1.262x |
| make_unmake_full | 1.012x | 0.995x | 1.370x |
| startpos | 1.000x | 1.000x | 1.334x |
| chess960_materialize | 1.001x | 0.993x | 1.403x |
| chess960_perft_d3 | 0.995x | 0.986x | 1.467x |
| polyglot_key | 0.999x | 0.998x | 21.823x |
| polyglot_recompute | 1.011x | 0.997x | 0.983x |
| repetition_key | 1.003x | 1.003x | 16.450x |
| corpus_count | 1.005x | 1.004x | 1.153x |
| corpus_materialize | 0.997x | 0.992x | 1.232x |
| corpus_uci_sorted | 1.009x | 1.003x | 1.822x |
| replay_trusted_native | 1.041x | 1.036x | 1.008x |
| replay_checked_native | 1.030x | 1.018x | 1.083x |

Cold first legal move list: 0.009 ms -> 0.009 ms (1.06x).
