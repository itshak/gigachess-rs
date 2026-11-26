| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.031x | 0.827x | 1.583x |
| materialize | 0.997x | 0.777x | 1.166x |
| perft_bulk_d4 | 0.983x | 0.784x | 1.221x |
| perft_full_d3 | 1.137x | 1.016x | 1.620x |
| fen_parse | 1.013x | 0.875x | 1.217x |
| fen_write_native_ep | 0.925x | 0.729x | 1.584x |
| snapshot | 0.930x | 0.622x | 0.915x |
| san_parse_all | 0.981x | 0.799x | 1.780x |
| san_render_native | 0.970x | 0.838x | 2.748x |
| apply_children_full | 1.002x | 0.893x | 1.074x |
| make_unmake_full | 0.953x | 0.788x | 1.363x |
| startpos | 0.972x | 0.972x | 1.720x |
| chess960_materialize | 0.832x | 0.776x | 1.440x |
| chess960_perft_d3 | 0.847x | 0.812x | 1.507x |
| polyglot_key | 0.930x | 0.888x | 40.158x |
| polyglot_recompute | 1.040x | 0.966x | 1.207x |
| repetition_key | 0.947x | 0.851x | 21.464x |
| corpus_count | 0.838x | 0.756x | 1.133x |
| corpus_materialize | 0.937x | 0.725x | 1.008x |
| corpus_uci_sorted | 0.830x | 0.830x | 1.410x |
| replay_trusted_native | 0.813x | 0.723x | 0.588x |
| replay_checked_native | 0.979x | 0.946x | 1.287x |

Cold first legal move list: 0.004 ms -> 0.006 ms (0.62x).
