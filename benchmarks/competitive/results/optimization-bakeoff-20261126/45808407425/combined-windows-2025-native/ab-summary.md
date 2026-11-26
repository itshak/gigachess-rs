| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.969x | 1.440x |
| materialize | 0.978x | 0.941x | 1.326x |
| perft_bulk_d4 | 0.958x | 0.917x | 1.174x |
| perft_full_d3 | 0.922x | 0.792x | 1.420x |
| fen_parse | 0.889x | 0.872x | 1.065x |
| fen_write_native_ep | 0.990x | 0.977x | 1.702x |
| snapshot | 0.962x | 0.941x | 1.398x |
| san_parse_all | 0.928x | 0.874x | 1.631x |
| san_render_native | 1.097x | 1.029x | 3.659x |
| apply_children_full | 0.925x | 0.879x | 1.118x |
| make_unmake_full | 0.919x | 0.902x | 1.021x |
| startpos | 1.199x | 1.199x | 1.264x |
| chess960_materialize | 0.943x | 0.924x | 1.267x |
| chess960_perft_d3 | 0.944x | 0.933x | 1.298x |
| polyglot_key | 0.999x | 0.998x | 31.563x |
| polyglot_recompute | 1.006x | 1.004x | 1.027x |
| repetition_key | 0.924x | 0.907x | 23.567x |
| corpus_count | 1.013x | 0.996x | 1.345x |
| corpus_materialize | 0.991x | 0.989x | 1.089x |
| corpus_uci_sorted | 0.988x | 0.986x | 1.528x |
| replay_trusted_native | 0.863x | 0.848x | 0.672x |
| replay_checked_native | 0.919x | 0.902x | 1.003x |

Cold first legal move list: 0.030 ms -> 0.032 ms (0.96x).
