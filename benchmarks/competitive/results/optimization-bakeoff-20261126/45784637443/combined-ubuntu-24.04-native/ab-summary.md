| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.056x | 0.992x | 1.488x |
| materialize | 1.034x | 1.003x | 1.542x |
| perft_bulk_d4 | 1.018x | 0.989x | 1.236x |
| perft_full_d3 | 0.973x | 0.920x | 1.466x |
| fen_parse | 1.130x | 1.101x | 1.191x |
| fen_write_native_ep | 1.001x | 0.998x | 1.916x |
| snapshot | 0.775x | 0.771x | 1.003x |
| san_parse_all | 1.001x | 0.990x | 1.892x |
| san_render_native | 0.987x | 0.984x | 2.096x |
| apply_children_full | 0.966x | 0.950x | 1.062x |
| make_unmake_full | 0.995x | 0.964x | 1.155x |
| startpos | 0.817x | 0.817x | 1.248x |
| chess960_materialize | 1.144x | 1.066x | 1.543x |
| chess960_perft_d3 | 1.063x | 1.010x | 1.481x |
| polyglot_key | 1.077x | 1.000x | 40.938x |
| polyglot_recompute | 1.006x | 0.992x | 0.985x |
| repetition_key | 1.230x | 1.129x | 22.948x |
| corpus_count | 1.032x | 1.023x | 1.397x |
| corpus_materialize | 1.018x | 1.015x | 1.099x |
| corpus_uci_sorted | 1.008x | 1.006x | 1.573x |
| replay_trusted_native | 0.997x | 0.988x | 0.711x |
| replay_checked_native | 1.002x | 1.001x | 1.070x |

Cold first legal move list: 0.009 ms -> 0.008 ms (1.07x).
