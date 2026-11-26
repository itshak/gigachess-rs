| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.968x | 0.927x | 1.107x |
| materialize | 0.996x | 0.985x | 1.213x |
| perft_bulk_d4 | 0.996x | 0.979x | 1.169x |
| perft_full_d3 | 0.995x | 0.973x | 0.863x |
| fen_parse | 0.943x | 0.898x | 1.164x |
| fen_write_native_ep | 0.992x | 0.978x | 0.956x |
| snapshot | 1.000x | 0.998x | 1.011x |
| san_parse_all | 1.003x | 0.993x | 1.874x |
| san_render_native | 1.025x | 0.987x | 2.281x |
| apply_children_full | 1.025x | 1.012x | 0.947x |
| make_unmake_full | 0.992x | 0.985x | 1.083x |
| startpos | 0.970x | 0.970x | 1.285x |
| chess960_materialize | 0.976x | 0.957x | 1.280x |
| chess960_perft_d3 | 0.994x | 0.986x | 1.261x |
| polyglot_key | 0.998x | 0.989x | 23.416x |
| polyglot_recompute | 0.985x | 0.978x | 1.005x |
| repetition_key | 1.008x | 0.988x | 18.306x |
| corpus_count | 0.950x | 0.922x | 1.195x |
| corpus_materialize | 0.981x | 0.976x | 1.181x |
| corpus_uci_sorted | 1.814x | 1.792x | 1.848x |
| replay_trusted_native | 0.994x | 0.976x | 0.608x |
| replay_checked_native | 1.025x | 1.016x | 1.026x |

Cold first legal move list: 0.015 ms -> 0.014 ms (1.07x).
