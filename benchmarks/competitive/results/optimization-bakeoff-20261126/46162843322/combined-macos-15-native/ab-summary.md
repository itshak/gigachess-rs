| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.048x | 1.022x | 1.574x |
| materialize | 1.011x | 0.899x | 1.051x |
| perft_bulk_d4 | 1.069x | 1.011x | 1.296x |
| perft_full_d3 | 1.158x | 0.931x | 1.501x |
| fen_parse | 0.980x | 0.893x | 1.201x |
| fen_write_native_ep | 1.049x | 0.963x | 1.781x |
| snapshot | 1.055x | 0.979x | 0.990x |
| san_parse_all | 1.010x | 0.873x | 1.687x |
| san_render_native | 1.070x | 0.892x | 2.821x |
| apply_children_full | 0.947x | 0.798x | 0.984x |
| make_unmake_full | 1.017x | 0.963x | 1.305x |
| startpos | 1.002x | 1.002x | 1.202x |
| chess960_materialize | 0.954x | 0.907x | 1.491x |
| chess960_perft_d3 | 0.985x | 0.956x | 1.506x |
| polyglot_key | 0.981x | 0.975x | 37.841x |
| polyglot_recompute | 1.545x | 1.339x | 1.540x |
| repetition_key | 1.007x | 0.985x | 19.430x |
| corpus_count | 1.060x | 0.924x | 1.379x |
| corpus_materialize | 1.091x | 1.081x | 1.134x |
| corpus_uci_sorted | 0.957x | 0.928x | 1.341x |
| replay_trusted_native | 1.024x | 1.016x | 0.691x |
| replay_checked_native | 0.938x | 0.854x | 1.070x |

Cold first legal move list: 0.005 ms -> 0.007 ms (0.71x).
