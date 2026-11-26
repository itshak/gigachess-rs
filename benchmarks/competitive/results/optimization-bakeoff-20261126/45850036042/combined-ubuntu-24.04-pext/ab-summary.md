| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.936x | 1.500x |
| materialize | 0.998x | 0.979x | 1.260x |
| perft_bulk_d4 | 0.979x | 0.932x | 1.371x |
| perft_full_d3 | 1.011x | 1.003x | 1.304x |
| fen_parse | 1.024x | 1.003x | 1.221x |
| fen_write_native_ep | 1.009x | 0.979x | 1.755x |
| snapshot | 0.998x | 0.996x | 1.246x |
| san_parse_all | 1.001x | 0.994x | 2.045x |
| san_render_native | 0.968x | 0.942x | 2.145x |
| apply_children_full | 0.992x | 0.977x | 1.289x |
| make_unmake_full | 0.972x | 0.940x | 1.247x |
| startpos | 1.001x | 1.001x | 1.252x |
| chess960_materialize | 0.993x | 0.991x | 1.403x |
| chess960_perft_d3 | 0.994x | 0.982x | 1.481x |
| polyglot_key | 0.864x | 0.779x | 24.270x |
| polyglot_recompute | 0.997x | 0.996x | 1.050x |
| repetition_key | 0.980x | 0.940x | 19.805x |
| corpus_count | 1.000x | 0.994x | 1.448x |
| corpus_materialize | 0.982x | 0.973x | 1.183x |
| corpus_uci_sorted | 1.001x | 1.000x | 1.740x |
| replay_trusted_native | 1.005x | 1.002x | 0.861x |
| replay_checked_native | 1.014x | 1.009x | 1.018x |

Cold first legal move list: 0.009 ms -> 0.009 ms (1.01x).
