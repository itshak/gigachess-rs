| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.005x | 0.975x | 1.557x |
| materialize | 1.002x | 0.995x | 1.620x |
| perft_bulk_d4 | 1.013x | 0.994x | 1.205x |
| perft_full_d3 | 1.007x | 0.987x | 1.397x |
| fen_parse | 1.008x | 0.993x | 1.119x |
| fen_write_native_ep | 1.000x | 0.956x | 1.941x |
| snapshot | 0.995x | 0.980x | 1.146x |
| san_parse_all | 1.002x | 0.992x | 1.728x |
| san_render_native | 0.997x | 0.981x | 2.112x |
| apply_children_full | 1.019x | 1.009x | 1.330x |
| make_unmake_full | 0.944x | 0.900x | 1.108x |
| startpos | 0.997x | 0.997x | 1.102x |
| chess960_materialize | 1.020x | 1.012x | 1.515x |
| chess960_perft_d3 | 1.010x | 1.006x | 1.488x |
| polyglot_key | 1.023x | 1.008x | 39.677x |
| polyglot_recompute | 1.061x | 1.049x | 1.022x |
| repetition_key | 0.970x | 0.928x | 22.772x |
| corpus_count | 1.059x | 1.021x | 1.137x |
| corpus_materialize | 1.004x | 1.003x | 1.081x |
| corpus_uci_sorted | 1.014x | 1.010x | 1.591x |
| replay_trusted_native | 0.985x | 0.976x | 0.879x |
| replay_checked_native | 0.978x | 0.977x | 1.029x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.84x).
