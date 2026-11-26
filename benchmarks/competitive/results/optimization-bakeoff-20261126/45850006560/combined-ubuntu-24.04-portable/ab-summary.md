| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.979x | 0.965x | 1.252x |
| materialize | 0.972x | 0.940x | 1.727x |
| perft_bulk_d4 | 0.974x | 0.950x | 1.154x |
| perft_full_d3 | 0.990x | 0.971x | 1.309x |
| fen_parse | 0.947x | 0.896x | 1.083x |
| fen_write_native_ep | 0.988x | 0.979x | 1.991x |
| snapshot | 0.800x | 0.798x | 1.000x |
| san_parse_all | 0.986x | 0.959x | 1.846x |
| san_render_native | 0.982x | 0.925x | 2.136x |
| apply_children_full | 0.964x | 0.888x | 1.345x |
| make_unmake_full | 0.908x | 0.875x | 1.118x |
| startpos | 1.000x | 1.000x | 1.290x |
| chess960_materialize | 0.934x | 0.878x | 1.457x |
| chess960_perft_d3 | 0.947x | 0.919x | 1.421x |
| polyglot_key | 0.896x | 0.801x | 37.808x |
| polyglot_recompute | 0.976x | 0.956x | 1.005x |
| repetition_key | 0.944x | 0.897x | 19.727x |
| corpus_count | 0.963x | 0.940x | 1.135x |
| corpus_materialize | 0.983x | 0.976x | 1.106x |
| corpus_uci_sorted | 1.003x | 1.003x | 1.537x |
| replay_trusted_native | 0.913x | 0.875x | 0.802x |
| replay_checked_native | 0.963x | 0.963x | 1.031x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.20x).
