| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.001x | 0.997x | 1.238x |
| materialize | 1.029x | 1.000x | 1.071x |
| perft_bulk_d4 | 1.011x | 0.989x | 0.988x |
| perft_full_d3 | 1.036x | 0.948x | 0.600x |
| fen_parse | 0.938x | 0.908x | 1.134x |
| fen_write_native_ep | 0.991x | 0.975x | 1.199x |
| snapshot | 0.779x | 0.762x | 0.779x |
| san_parse_all | 0.988x | 0.932x | 0.959x |
| san_render_native | 0.999x | 0.985x | 1.112x |
| apply_children_full | 0.972x | 0.955x | 0.667x |
| make_unmake_full | 0.998x | 0.986x | 0.828x |
| startpos | 0.887x | 0.887x | 0.776x |
| chess960_materialize | 1.010x | 0.997x | 1.258x |
| chess960_perft_d3 | 1.002x | 0.994x | 1.023x |
| polyglot_key | 1.001x | 0.999x | 23.230x |
| polyglot_recompute | 1.012x | 0.999x | 1.052x |
| repetition_key | 1.001x | 0.999x | 13.045x |
| corpus_count | 0.995x | 0.994x | 1.254x |
| corpus_materialize | 0.982x | 0.971x | 1.190x |
| corpus_uci_sorted | 0.976x | 0.969x | 0.635x |
| replay_trusted_native | 0.998x | 0.982x | 0.541x |
| replay_checked_native | 0.995x | 0.993x | 0.722x |

Cold first legal move list: 154.919 ms → 156.706 ms (0.99x).
