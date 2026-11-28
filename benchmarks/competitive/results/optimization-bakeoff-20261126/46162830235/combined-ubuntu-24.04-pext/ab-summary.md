| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.040x | 1.001x | 1.619x |
| materialize | 0.945x | 0.890x | 1.524x |
| perft_bulk_d4 | 1.050x | 1.022x | 1.257x |
| perft_full_d3 | 1.239x | 1.050x | 1.687x |
| fen_parse | 0.991x | 0.956x | 1.090x |
| fen_write_native_ep | 0.994x | 0.967x | 1.947x |
| snapshot | 1.000x | 0.994x | 1.323x |
| san_parse_all | 1.020x | 1.007x | 1.702x |
| san_render_native | 0.985x | 0.979x | 2.111x |
| apply_children_full | 1.003x | 0.993x | 1.326x |
| make_unmake_full | 0.968x | 0.916x | 1.158x |
| startpos | 0.888x | 0.888x | 0.989x |
| chess960_materialize | 1.007x | 1.005x | 1.532x |
| chess960_perft_d3 | 1.008x | 0.993x | 1.483x |
| polyglot_key | 1.003x | 0.996x | 39.080x |
| polyglot_recompute | 1.693x | 1.374x | 1.632x |
| repetition_key | 1.146x | 0.928x | 22.085x |
| corpus_count | 1.219x | 1.118x | 1.300x |
| corpus_materialize | 1.001x | 0.996x | 1.083x |
| corpus_uci_sorted | 1.012x | 1.000x | 1.570x |
| replay_trusted_native | 1.009x | 1.006x | 0.863x |
| replay_checked_native | 0.990x | 0.987x | 1.149x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.97x).
