| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.083x |
| materialize | 0.999x | 0.996x | 1.190x |
| perft_bulk_d4 | 1.000x | 0.995x | 0.908x |
| perft_full_d3 | 1.006x | 1.002x | 0.615x |
| fen_parse | 1.006x | 1.003x | 1.223x |
| fen_write_native_ep | 1.009x | 1.004x | 1.059x |
| snapshot | 1.019x | 1.001x | 0.785x |
| san_parse_all | 1.014x | 1.009x | 0.927x |
| san_render_native | 1.013x | 1.009x | 1.159x |
| apply_children_full | 1.011x | 1.006x | 0.665x |
| make_unmake_full | 1.026x | 1.019x | 0.915x |
| startpos | 1.027x | 1.027x | 0.836x |
| chess960_materialize | 1.001x | 0.998x | 1.112x |
| chess960_perft_d3 | 1.006x | 1.003x | 0.849x |
| polyglot_key | 1.006x | 0.999x | 23.127x |
| polyglot_recompute | 0.998x | 0.994x | 1.132x |
| repetition_key | 1.004x | 1.000x | 13.912x |
| corpus_count | 1.019x | 1.006x | 1.188x |
| corpus_materialize | 1.003x | 1.003x | 1.125x |
| corpus_uci_sorted | 0.999x | 0.998x | 0.571x |
| replay_trusted_native | 1.009x | 1.007x | 0.328x |
| replay_checked_native | 1.015x | 1.014x | 0.668x |

Cold first legal move list: 153.853 ms → 153.945 ms (1.00x).
