| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.988x | 1.570x |
| materialize | 1.046x | 1.025x | 1.571x |
| perft_bulk_d4 | 1.001x | 0.984x | 1.195x |
| perft_full_d3 | 0.988x | 0.968x | 1.425x |
| fen_parse | 1.052x | 0.799x | 1.039x |
| fen_write_native_ep | 0.987x | 0.958x | 2.115x |
| snapshot | 0.995x | 0.989x | 0.992x |
| san_parse_all | 0.991x | 0.977x | 1.874x |
| san_render_native | 0.976x | 0.960x | 2.099x |
| apply_children_full | 1.018x | 1.006x | 1.344x |
| make_unmake_full | 0.932x | 0.887x | 1.086x |
| startpos | 0.902x | 0.902x | 1.002x |
| chess960_materialize | 0.981x | 0.963x | 1.495x |
| chess960_perft_d3 | 1.003x | 1.001x | 1.473x |
| polyglot_key | 0.996x | 0.995x | 41.960x |
| polyglot_recompute | 1.105x | 1.071x | 1.107x |
| repetition_key | 1.193x | 0.997x | 24.659x |
| corpus_count | 1.078x | 1.021x | 1.139x |
| corpus_materialize | 1.010x | 1.009x | 1.092x |
| corpus_uci_sorted | 1.014x | 1.012x | 1.577x |
| replay_trusted_native | 0.996x | 0.958x | 0.902x |
| replay_checked_native | 1.037x | 1.006x | 1.149x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.05x).
