| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.924x | 1.157x |
| materialize | 1.035x | 0.883x | 1.108x |
| perft_bulk_d4 | 1.130x | 0.897x | 1.271x |
| perft_full_d3 | 1.151x | 0.972x | 0.821x |
| fen_parse | 1.150x | 0.954x | 1.229x |
| fen_write_native_ep | 1.041x | 0.873x | 1.166x |
| snapshot | 1.090x | 0.883x | 0.950x |
| san_parse_all | 1.333x | 1.191x | 1.910x |
| san_render_native | 1.126x | 0.913x | 2.602x |
| apply_children_full | 1.049x | 0.875x | 0.755x |
| make_unmake_full | 1.259x | 1.022x | 1.101x |
| startpos | 0.950x | 0.950x | 1.272x |
| chess960_materialize | 1.312x | 1.083x | 1.356x |
| chess960_perft_d3 | 1.245x | 1.080x | 1.456x |
| polyglot_key | 1.155x | 1.119x | 38.236x |
| polyglot_recompute | 1.153x | 1.099x | 0.973x |
| repetition_key | 1.100x | 0.981x | 19.817x |
| corpus_count | 1.234x | 1.132x | 1.186x |
| corpus_materialize | 1.200x | 1.018x | 1.122x |
| corpus_uci_sorted | 1.076x | 1.003x | 0.883x |
| replay_trusted_native | 1.047x | 0.880x | 0.437x |
| replay_checked_native | 1.275x | 1.233x | 0.996x |

Cold first legal move list: 0.013 ms -> 0.015 ms (0.87x).
