| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 0.987x | 1.570x |
| materialize | 1.020x | 1.014x | 1.347x |
| perft_bulk_d4 | 1.040x | 1.018x | 1.452x |
| perft_full_d3 | 1.131x | 0.998x | 1.500x |
| fen_parse | 1.019x | 0.945x | 1.243x |
| fen_write_native_ep | 0.995x | 0.988x | 1.565x |
| snapshot | 0.983x | 0.981x | 0.958x |
| san_parse_all | 0.992x | 0.984x | 1.823x |
| san_render_native | 1.000x | 0.993x | 2.667x |
| apply_children_full | 0.992x | 0.961x | 1.147x |
| make_unmake_full | 1.025x | 1.010x | 1.589x |
| startpos | 1.001x | 1.001x | 1.329x |
| chess960_materialize | 1.012x | 1.011x | 1.515x |
| chess960_perft_d3 | 1.021x | 1.014x | 1.521x |
| polyglot_key | 0.987x | 0.972x | 25.101x |
| polyglot_recompute | 1.305x | 1.214x | 1.414x |
| repetition_key | 0.994x | 0.988x | 19.122x |
| corpus_count | 1.069x | 1.034x | 1.471x |
| corpus_materialize | 0.995x | 0.994x | 1.215x |
| corpus_uci_sorted | 0.998x | 0.998x | 1.455x |
| replay_trusted_native | 0.999x | 0.993x | 0.626x |
| replay_checked_native | 1.013x | 1.007x | 1.150x |

Cold first legal move list: 0.005 ms -> 0.007 ms (0.83x).
