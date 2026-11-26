| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.083x | 0.978x | 1.235x |
| materialize | 1.113x | 0.874x | 1.027x |
| perft_bulk_d4 | 1.101x | 0.936x | 1.178x |
| perft_full_d3 | 1.010x | 0.836x | 1.262x |
| fen_parse | 0.960x | 0.778x | 1.165x |
| fen_write_native_ep | 0.979x | 0.771x | 1.854x |
| snapshot | 1.237x | 0.854x | 1.039x |
| san_parse_all | 1.063x | 0.969x | 1.664x |
| san_render_native | 1.025x | 0.723x | 2.587x |
| apply_children_full | 1.034x | 0.867x | 1.058x |
| make_unmake_full | 1.107x | 0.921x | 1.287x |
| startpos | 1.225x | 1.225x | 1.185x |
| chess960_materialize | 1.153x | 1.143x | 1.357x |
| chess960_perft_d3 | 1.192x | 1.166x | 1.257x |
| polyglot_key | 1.315x | 1.205x | 38.776x |
| polyglot_recompute | 1.119x | 1.087x | 1.031x |
| repetition_key | 1.322x | 1.293x | 19.301x |
| corpus_count | 1.116x | 0.986x | 0.955x |
| corpus_materialize | 1.130x | 1.035x | 1.045x |
| corpus_uci_sorted | 1.230x | 1.221x | 1.468x |
| replay_trusted_native | 1.533x | 0.996x | 0.665x |
| replay_checked_native | 1.056x | 1.006x | 1.123x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.03x).
