| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.055x | 1.026x | 1.555x |
| materialize | 1.068x | 0.934x | 1.250x |
| perft_bulk_d4 | 1.042x | 1.023x | 1.342x |
| perft_full_d3 | 1.309x | 1.054x | 1.625x |
| fen_parse | 0.926x | 0.885x | 1.148x |
| fen_write_native_ep | 1.004x | 0.991x | 1.479x |
| snapshot | 1.008x | 0.972x | 1.491x |
| san_parse_all | 1.012x | 0.996x | 1.778x |
| san_render_native | 0.992x | 0.938x | 2.283x |
| apply_children_full | 0.996x | 0.972x | 1.358x |
| make_unmake_full | 1.023x | 0.978x | 1.205x |
| startpos | 0.981x | 0.981x | 1.251x |
| chess960_materialize | 0.978x | 0.965x | 1.480x |
| chess960_perft_d3 | 0.991x | 0.974x | 1.532x |
| polyglot_key | 1.024x | 0.995x | 37.082x |
| polyglot_recompute | 1.614x | 1.380x | 1.661x |
| repetition_key | 0.833x | 0.791x | 23.489x |
| corpus_count | 1.079x | 1.042x | 1.292x |
| corpus_materialize | 1.004x | 1.002x | 1.151x |
| corpus_uci_sorted | 1.003x | 0.990x | 1.686x |
| replay_trusted_native | 1.030x | 1.021x | 0.767x |
| replay_checked_native | 0.998x | 0.990x | 1.216x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.96x).
