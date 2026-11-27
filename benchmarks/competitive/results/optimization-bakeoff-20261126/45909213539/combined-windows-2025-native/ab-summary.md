| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.992x | 0.927x | 1.512x |
| materialize | 0.991x | 0.979x | 1.350x |
| perft_bulk_d4 | 0.988x | 0.913x | 1.175x |
| perft_full_d3 | 1.010x | 0.931x | 1.431x |
| fen_parse | 0.995x | 0.892x | 1.260x |
| fen_write_native_ep | 1.004x | 0.950x | 1.681x |
| snapshot | 0.929x | 0.861x | 0.986x |
| san_parse_all | 1.004x | 0.994x | 1.769x |
| san_render_native | 0.998x | 0.978x | 4.229x |
| apply_children_full | 0.997x | 0.945x | 1.369x |
| make_unmake_full | 1.008x | 0.989x | 1.213x |
| startpos | 0.993x | 0.993x | 1.023x |
| chess960_materialize | 0.956x | 0.905x | 1.297x |
| chess960_perft_d3 | 0.916x | 0.849x | 1.436x |
| polyglot_key | 0.980x | 0.977x | 39.865x |
| polyglot_recompute | 0.897x | 0.814x | 0.930x |
| repetition_key | 0.995x | 0.974x | 26.461x |
| corpus_count | 1.005x | 1.002x | 1.004x |
| corpus_materialize | 0.994x | 0.991x | 1.110x |
| corpus_uci_sorted | 0.923x | 0.912x | 1.438x |
| replay_trusted_native | 0.989x | 0.972x | 0.836x |
| replay_checked_native | 0.975x | 0.923x | 1.177x |

Cold first legal move list: 0.016 ms -> 0.015 ms (1.05x).
