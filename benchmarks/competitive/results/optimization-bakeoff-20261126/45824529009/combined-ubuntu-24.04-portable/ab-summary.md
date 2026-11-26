| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.997x | 1.203x |
| materialize | 1.019x | 1.003x | 1.268x |
| perft_bulk_d4 | 0.997x | 0.979x | 1.150x |
| perft_full_d3 | 1.000x | 0.974x | 1.295x |
| fen_parse | 1.048x | 1.011x | 1.230x |
| fen_write_native_ep | 1.008x | 0.999x | 1.680x |
| snapshot | 0.998x | 0.996x | 0.978x |
| san_parse_all | 1.002x | 1.000x | 1.997x |
| san_render_native | 1.008x | 0.996x | 2.352x |
| apply_children_full | 1.012x | 0.987x | 1.385x |
| make_unmake_full | 1.035x | 1.002x | 1.392x |
| startpos | 1.002x | 1.002x | 1.335x |
| chess960_materialize | 0.998x | 0.998x | 1.339x |
| chess960_perft_d3 | 0.996x | 0.991x | 1.469x |
| polyglot_key | 1.088x | 1.007x | 27.495x |
| polyglot_recompute | 1.029x | 1.016x | 1.021x |
| repetition_key | 0.990x | 0.975x | 17.905x |
| corpus_count | 1.019x | 1.005x | 1.192x |
| corpus_materialize | 1.013x | 1.011x | 1.133x |
| corpus_uci_sorted | 1.016x | 1.012x | 1.785x |
| replay_trusted_native | 1.040x | 1.027x | 0.953x |
| replay_checked_native | 1.053x | 1.046x | 1.074x |

Cold first legal move list: 0.008 ms -> 0.008 ms (1.09x).
