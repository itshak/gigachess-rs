| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.097x | 0.923x | 1.505x |
| materialize | 0.960x | 0.869x | 1.544x |
| perft_bulk_d4 | 1.164x | 1.020x | 1.170x |
| perft_full_d3 | 0.987x | 0.900x | 1.477x |
| fen_parse | 1.075x | 1.029x | 1.061x |
| fen_write_native_ep | 0.967x | 0.904x | 2.032x |
| snapshot | 0.999x | 0.994x | 0.999x |
| san_parse_all | 0.893x | 0.861x | 1.694x |
| san_render_native | 1.003x | 1.000x | 2.115x |
| apply_children_full | 1.000x | 0.996x | 1.304x |
| make_unmake_full | 1.002x | 0.992x | 1.152x |
| startpos | 0.999x | 0.999x | 0.998x |
| chess960_materialize | 1.065x | 1.029x | 1.513x |
| chess960_perft_d3 | 1.070x | 1.061x | 1.503x |
| polyglot_key | 1.077x | 1.002x | 40.789x |
| polyglot_recompute | 1.000x | 0.997x | 1.068x |
| repetition_key | 0.875x | 0.779x | 17.583x |
| corpus_count | 1.274x | 1.055x | 1.105x |
| corpus_materialize | 1.047x | 1.038x | 1.073x |
| corpus_uci_sorted | 0.987x | 0.987x | 1.528x |
| replay_trusted_native | 1.003x | 1.001x | 0.902x |
| replay_checked_native | 1.016x | 1.001x | 1.083x |

Cold first legal move list: 0.005 ms -> 0.005 ms (0.97x).
