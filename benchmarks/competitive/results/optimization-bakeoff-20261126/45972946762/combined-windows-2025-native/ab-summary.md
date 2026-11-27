| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 0.995x | 1.544x |
| materialize | 1.002x | 0.968x | 1.142x |
| perft_bulk_d4 | 0.997x | 0.978x | 1.420x |
| perft_full_d3 | 0.984x | 0.929x | 1.278x |
| fen_parse | 1.067x | 1.036x | 1.538x |
| fen_write_native_ep | 1.012x | 0.992x | 1.594x |
| snapshot | 1.001x | 0.981x | 1.161x |
| san_parse_all | 0.987x | 0.974x | 1.945x |
| san_render_native | 0.987x | 0.976x | 3.260x |
| apply_children_full | 1.033x | 0.985x | 1.316x |
| make_unmake_full | 1.001x | 0.973x | 1.302x |
| startpos | 1.002x | 1.002x | 0.999x |
| chess960_materialize | 1.003x | 1.002x | 1.356x |
| chess960_perft_d3 | 1.024x | 1.021x | 1.612x |
| polyglot_key | 1.008x | 1.000x | 20.229x |
| polyglot_recompute | 0.987x | 0.973x | 0.957x |
| repetition_key | 1.014x | 1.002x | 19.139x |
| corpus_count | 1.028x | 1.008x | 1.327x |
| corpus_materialize | 1.004x | 1.001x | 1.194x |
| corpus_uci_sorted | 1.041x | 1.005x | 1.563x |
| replay_trusted_native | 0.984x | 0.966x | 1.104x |
| replay_checked_native | 1.006x | 0.983x | 1.072x |

Cold first legal move list: 0.019 ms -> 0.018 ms (1.06x).
