| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.992x | 1.550x |
| materialize | 0.953x | 0.916x | 1.524x |
| perft_bulk_d4 | 0.990x | 0.982x | 1.182x |
| perft_full_d3 | 1.001x | 0.997x | 1.284x |
| fen_parse | 1.003x | 0.961x | 1.119x |
| fen_write_native_ep | 0.996x | 0.989x | 2.096x |
| snapshot | 0.998x | 0.991x | 1.151x |
| san_parse_all | 0.963x | 0.943x | 1.827x |
| san_render_native | 1.001x | 0.996x | 2.123x |
| apply_children_full | 0.986x | 0.972x | 1.292x |
| make_unmake_full | 0.989x | 0.969x | 1.148x |
| startpos | 0.998x | 0.998x | 1.124x |
| chess960_materialize | 0.995x | 0.994x | 1.500x |
| chess960_perft_d3 | 1.007x | 1.002x | 1.483x |
| polyglot_key | 0.931x | 0.863x | 37.308x |
| polyglot_recompute | 0.633x | 0.560x | 0.611x |
| repetition_key | 0.970x | 0.935x | 18.880x |
| corpus_count | 1.004x | 1.004x | 1.053x |
| corpus_materialize | 1.003x | 0.999x | 1.090x |
| corpus_uci_sorted | 0.995x | 0.987x | 1.546x |
| replay_trusted_native | 0.966x | 0.954x | 0.866x |
| replay_checked_native | 0.968x | 0.945x | 1.074x |

Cold first legal move list: 0.005 ms -> 0.007 ms (0.71x).
