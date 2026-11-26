| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.788x | 1.589x |
| materialize | 0.937x | 0.821x | 1.025x |
| perft_bulk_d4 | 0.982x | 0.697x | 1.249x |
| perft_full_d3 | 1.118x | 0.964x | 1.475x |
| fen_parse | 1.086x | 0.891x | 1.292x |
| fen_write_native_ep | 1.034x | 0.887x | 1.775x |
| snapshot | 1.014x | 0.968x | 1.061x |
| san_parse_all | 1.077x | 0.968x | 1.731x |
| san_render_native | 0.987x | 0.884x | 2.668x |
| apply_children_full | 0.974x | 0.885x | 1.106x |
| make_unmake_full | 0.952x | 0.837x | 1.377x |
| startpos | 0.966x | 0.966x | 1.176x |
| chess960_materialize | 1.052x | 1.003x | 1.546x |
| chess960_perft_d3 | 1.059x | 1.052x | 1.525x |
| polyglot_key | 0.999x | 0.975x | 38.188x |
| polyglot_recompute | 1.013x | 0.997x | 0.953x |
| repetition_key | 1.142x | 1.109x | 20.125x |
| corpus_count | 1.213x | 1.040x | 1.171x |
| corpus_materialize | 1.004x | 0.981x | 1.119x |
| corpus_uci_sorted | 1.037x | 1.006x | 1.356x |
| replay_trusted_native | 1.024x | 1.000x | 0.661x |
| replay_checked_native | 1.086x | 0.874x | 1.143x |

Cold first legal move list: 0.003 ms -> 0.006 ms (0.56x).
