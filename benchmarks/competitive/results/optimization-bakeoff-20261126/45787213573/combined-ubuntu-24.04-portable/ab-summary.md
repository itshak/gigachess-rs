| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.991x | 1.148x |
| materialize | 0.940x | 0.911x | 1.226x |
| perft_bulk_d4 | 1.000x | 0.984x | 1.162x |
| perft_full_d3 | 0.955x | 0.871x | 1.301x |
| fen_parse | 1.002x | 0.971x | 1.243x |
| fen_write_native_ep | 0.993x | 0.986x | 1.551x |
| snapshot | 0.999x | 0.994x | 1.002x |
| san_parse_all | 0.970x | 0.961x | 1.762x |
| san_render_native | 0.949x | 0.904x | 2.262x |
| apply_children_full | 0.942x | 0.916x | 0.960x |
| make_unmake_full | 1.392x | 1.223x | 1.150x |
| startpos | 0.999x | 0.999x | 1.280x |
| chess960_materialize | 0.980x | 0.955x | 1.335x |
| chess960_perft_d3 | 1.012x | 0.999x | 1.284x |
| polyglot_key | 1.014x | 0.996x | 32.801x |
| polyglot_recompute | 0.986x | 0.984x | 1.008x |
| repetition_key | 1.018x | 1.000x | 20.879x |
| corpus_count | 0.998x | 0.997x | 1.122x |
| corpus_materialize | 1.007x | 0.995x | 1.260x |
| corpus_uci_sorted | 1.004x | 0.990x | 1.591x |
| replay_trusted_native | 0.972x | 0.970x | 0.557x |
| replay_checked_native | 1.086x | 1.079x | 1.065x |

Cold first legal move list: 0.015 ms -> 0.014 ms (1.10x).
