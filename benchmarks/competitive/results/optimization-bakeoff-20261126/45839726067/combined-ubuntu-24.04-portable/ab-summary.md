| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.984x | 1.295x |
| materialize | 1.002x | 0.991x | 1.326x |
| perft_bulk_d4 | 0.995x | 0.989x | 1.264x |
| perft_full_d3 | 0.985x | 0.922x | 0.886x |
| fen_parse | 1.023x | 0.929x | 1.310x |
| fen_write_native_ep | 0.992x | 0.974x | 1.722x |
| snapshot | 0.998x | 0.996x | 0.974x |
| san_parse_all | 1.008x | 0.990x | 2.140x |
| san_render_native | 0.988x | 0.970x | 2.277x |
| apply_children_full | 1.050x | 1.013x | 1.300x |
| make_unmake_full | 1.009x | 0.990x | 1.393x |
| startpos | 0.999x | 0.999x | 1.335x |
| chess960_materialize | 0.958x | 0.954x | 1.599x |
| chess960_perft_d3 | 0.997x | 0.987x | 1.583x |
| polyglot_key | 1.000x | 0.999x | 21.902x |
| polyglot_recompute | 1.054x | 0.966x | 1.049x |
| repetition_key | 0.998x | 0.989x | 20.193x |
| corpus_count | 1.000x | 0.999x | 1.230x |
| corpus_materialize | 1.000x | 0.998x | 1.350x |
| corpus_uci_sorted | 0.994x | 0.987x | 1.594x |
| replay_trusted_native | 0.887x | 0.776x | 0.854x |
| replay_checked_native | 1.067x | 1.050x | 1.143x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.02x).
