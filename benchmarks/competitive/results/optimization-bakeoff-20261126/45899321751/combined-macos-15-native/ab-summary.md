| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.160x | 0.923x | 1.593x |
| materialize | 1.108x | 0.932x | 1.200x |
| perft_bulk_d4 | 1.160x | 0.858x | 1.317x |
| perft_full_d3 | 1.052x | 0.845x | 1.249x |
| fen_parse | 1.045x | 0.907x | 1.153x |
| fen_write_native_ep | 1.149x | 0.926x | 1.809x |
| snapshot | 1.076x | 0.861x | 1.061x |
| san_parse_all | 1.052x | 0.900x | 1.689x |
| san_render_native | 1.016x | 0.718x | 2.842x |
| apply_children_full | 0.974x | 0.752x | 1.067x |
| make_unmake_full | 0.969x | 0.564x | 1.236x |
| startpos | 0.821x | 0.821x | 1.376x |
| chess960_materialize | 0.772x | 0.718x | 1.641x |
| chess960_perft_d3 | 0.756x | 0.696x | 1.362x |
| polyglot_key | 0.996x | 0.889x | 38.954x |
| polyglot_recompute | 1.002x | 0.933x | 1.012x |
| repetition_key | 1.069x | 1.040x | 19.640x |
| corpus_count | 1.153x | 1.066x | 1.435x |
| corpus_materialize | 1.049x | 1.006x | 1.193x |
| corpus_uci_sorted | 0.962x | 0.824x | 1.418x |
| replay_trusted_native | 1.034x | 0.982x | 0.627x |
| replay_checked_native | 0.979x | 0.902x | 1.202x |

Cold first legal move list: 0.007 ms -> 0.005 ms (1.65x).
