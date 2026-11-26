| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.998x | 1.090x |
| materialize | 1.003x | 1.000x | 1.170x |
| perft_bulk_d4 | 1.050x | 1.030x | 0.928x |
| perft_full_d3 | 1.061x | 1.042x | 0.637x |
| fen_parse | 1.000x | 0.994x | 1.197x |
| fen_write_native_ep | 0.997x | 0.991x | 1.047x |
| snapshot | 0.985x | 0.977x | 0.745x |
| san_parse_all | 1.000x | 0.990x | 0.929x |
| san_render_native | 0.996x | 0.989x | 1.143x |
| apply_children_full | 1.004x | 0.996x | 0.656x |
| make_unmake_full | 0.998x | 0.991x | 0.892x |
| startpos | 1.010x | 1.010x | 0.765x |
| chess960_materialize | 1.003x | 1.000x | 1.104x |
| chess960_perft_d3 | 1.019x | 1.015x | 0.856x |
| polyglot_key | 0.992x | 0.982x | 22.428x |
| polyglot_recompute | 0.999x | 0.998x | 1.128x |
| repetition_key | 0.995x | 0.986x | 13.742x |
| corpus_count | 0.996x | 0.991x | 1.209x |
| corpus_materialize | 0.989x | 0.984x | 1.111x |
| corpus_uci_sorted | 0.990x | 0.988x | 0.576x |
| replay_trusted_native | 1.001x | 1.000x | 0.322x |
| replay_checked_native | 1.002x | 0.994x | 0.669x |

Cold first legal move list: 153.932 ms → 154.044 ms (1.00x).
