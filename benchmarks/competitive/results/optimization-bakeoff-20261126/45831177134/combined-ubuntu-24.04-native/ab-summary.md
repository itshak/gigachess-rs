| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.979x | 0.951x | 1.328x |
| materialize | 1.048x | 1.023x | 1.439x |
| perft_bulk_d4 | 0.990x | 0.964x | 1.000x |
| perft_full_d3 | 1.011x | 0.997x | 1.500x |
| fen_parse | 1.086x | 1.028x | 1.131x |
| fen_write_native_ep | 0.948x | 0.897x | 1.804x |
| snapshot | 0.999x | 0.993x | 1.154x |
| san_parse_all | 0.975x | 0.950x | 1.800x |
| san_render_native | 0.973x | 0.956x | 2.088x |
| apply_children_full | 1.025x | 1.018x | 1.329x |
| make_unmake_full | 1.010x | 0.927x | 1.180x |
| startpos | 0.892x | 0.892x | 1.122x |
| chess960_materialize | 0.987x | 0.983x | 1.320x |
| chess960_perft_d3 | 1.008x | 0.994x | 1.364x |
| polyglot_key | 0.999x | 0.998x | 39.615x |
| polyglot_recompute | 0.980x | 0.969x | 0.989x |
| repetition_key | 0.938x | 0.767x | 24.873x |
| corpus_count | 0.991x | 0.984x | 0.823x |
| corpus_materialize | 0.995x | 0.993x | 1.008x |
| corpus_uci_sorted | 1.020x | 1.012x | 1.543x |
| replay_trusted_native | 1.030x | 0.988x | 0.913x |
| replay_checked_native | 1.024x | 1.021x | 1.088x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.93x).
