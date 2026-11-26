| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.023x | 1.006x | 1.185x |
| materialize | 1.066x | 1.042x | 1.270x |
| perft_bulk_d4 | 0.859x | 0.782x | 1.023x |
| perft_full_d3 | 1.046x | 0.859x | 1.176x |
| fen_parse | 0.997x | 0.951x | 1.281x |
| fen_write_native_ep | 0.989x | 0.980x | 1.196x |
| snapshot | 1.062x | 1.007x | 0.988x |
| san_parse_all | 0.860x | 0.835x | 1.878x |
| san_render_native | 0.902x | 0.810x | 2.150x |
| apply_children_full | 0.732x | 0.630x | 0.798x |
| make_unmake_full | 0.776x | 0.641x | 0.987x |
| startpos | 1.079x | 1.079x | 1.410x |
| chess960_materialize | 1.112x | 1.104x | 1.495x |
| chess960_perft_d3 | 0.919x | 0.829x | 1.300x |
| polyglot_key | 1.021x | 1.001x | 22.296x |
| polyglot_recompute | 0.998x | 0.996x | 0.990x |
| repetition_key | 0.980x | 0.959x | 20.387x |
| corpus_count | 0.997x | 0.992x | 1.222x |
| corpus_materialize | 1.003x | 1.001x | 1.314x |
| corpus_uci_sorted | 0.995x | 0.991x | 1.605x |
| replay_trusted_native | 0.833x | 0.813x | 0.573x |
| replay_checked_native | 0.926x | 0.920x | 0.923x |

Cold first legal move list: 0.011 ms -> 0.012 ms (0.98x).
