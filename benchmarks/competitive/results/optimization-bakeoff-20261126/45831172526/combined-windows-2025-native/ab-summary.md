| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.977x | 0.803x | 1.343x |
| materialize | 0.961x | 0.913x | 1.279x |
| perft_bulk_d4 | 0.963x | 0.927x | 1.008x |
| perft_full_d3 | 0.933x | 0.897x | 1.501x |
| fen_parse | 1.073x | 1.031x | 1.393x |
| fen_write_native_ep | 0.985x | 0.955x | 1.658x |
| snapshot | 0.846x | 0.734x | 0.966x |
| san_parse_all | 0.903x | 0.850x | 1.627x |
| san_render_native | 1.021x | 0.997x | 4.444x |
| apply_children_full | 0.964x | 0.814x | 1.313x |
| make_unmake_full | 0.966x | 0.871x | 1.198x |
| startpos | 1.003x | 1.003x | 1.002x |
| chess960_materialize | 0.954x | 0.916x | 1.217x |
| chess960_perft_d3 | 0.973x | 0.952x | 1.347x |
| polyglot_key | 0.980x | 0.960x | 40.500x |
| polyglot_recompute | 1.034x | 0.990x | 1.036x |
| repetition_key | 0.975x | 0.824x | 27.432x |
| corpus_count | 0.993x | 0.975x | 0.812x |
| corpus_materialize | 1.010x | 0.975x | 1.016x |
| corpus_uci_sorted | 1.083x | 0.988x | 1.525x |
| replay_trusted_native | 0.985x | 0.968x | 0.841x |
| replay_checked_native | 1.012x | 0.993x | 1.094x |

Cold first legal move list: 0.017 ms -> 0.016 ms (1.05x).
