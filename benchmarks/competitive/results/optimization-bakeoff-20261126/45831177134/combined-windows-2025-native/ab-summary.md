| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.984x | 0.906x | 1.347x |
| materialize | 0.996x | 0.987x | 1.074x |
| perft_bulk_d4 | 1.022x | 0.958x | 1.287x |
| perft_full_d3 | 0.955x | 0.941x | 1.046x |
| fen_parse | 1.160x | 1.121x | 1.440x |
| fen_write_native_ep | 0.954x | 0.941x | 1.518x |
| snapshot | 0.997x | 0.988x | 1.182x |
| san_parse_all | 1.014x | 1.001x | 1.984x |
| san_render_native | 0.952x | 0.892x | 3.166x |
| apply_children_full | 1.019x | 1.001x | 1.300x |
| make_unmake_full | 1.004x | 0.952x | 1.296x |
| startpos | 1.000x | 1.000x | 1.000x |
| chess960_materialize | 1.002x | 0.995x | 1.251x |
| chess960_perft_d3 | 0.998x | 0.996x | 1.473x |
| polyglot_key | 0.996x | 0.979x | 20.225x |
| polyglot_recompute | 1.036x | 1.029x | 0.980x |
| repetition_key | 1.015x | 0.999x | 19.215x |
| corpus_count | 1.001x | 0.992x | 1.209x |
| corpus_materialize | 1.097x | 1.010x | 1.123x |
| corpus_uci_sorted | 1.022x | 1.003x | 1.509x |
| replay_trusted_native | 1.155x | 1.048x | 1.178x |
| replay_checked_native | 1.023x | 1.022x | 1.023x |

Cold first legal move list: 0.014 ms -> 0.014 ms (1.03x).
