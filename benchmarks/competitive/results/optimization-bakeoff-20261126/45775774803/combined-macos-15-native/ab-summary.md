| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.069x | 0.972x | 1.249x |
| materialize | 0.979x | 0.831x | 1.001x |
| perft_bulk_d4 | 0.929x | 0.757x | 1.186x |
| perft_full_d3 | 1.065x | 0.919x | 0.808x |
| fen_parse | 0.982x | 0.769x | 1.105x |
| fen_write_native_ep | 0.955x | 0.624x | 1.198x |
| snapshot | 0.866x | 0.420x | 0.761x |
| san_parse_all | 1.031x | 0.945x | 1.667x |
| san_render_native | 1.006x | 0.816x | 2.638x |
| apply_children_full | 1.057x | 0.827x | 0.781x |
| make_unmake_full | 0.973x | 0.774x | 1.012x |
| startpos | 1.027x | 1.027x | 1.255x |
| chess960_materialize | 1.220x | 1.157x | 1.251x |
| chess960_perft_d3 | 1.119x | 1.017x | 1.433x |
| polyglot_key | 0.915x | 0.765x | 37.464x |
| polyglot_recompute | 0.969x | 0.869x | 1.028x |
| repetition_key | 0.920x | 0.768x | 19.204x |
| corpus_count | 1.063x | 0.997x | 1.359x |
| corpus_materialize | 1.128x | 1.062x | 1.124x |
| corpus_uci_sorted | 0.952x | 0.915x | 0.994x |
| replay_trusted_native | 0.825x | 0.721x | 0.418x |
| replay_checked_native | 0.965x | 0.954x | 1.026x |

Cold first legal move list: 0.013 ms -> 0.012 ms (1.13x).
