| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.019x | 0.941x | 1.382x |
| materialize | 1.012x | 0.976x | 1.246x |
| perft_bulk_d4 | 0.925x | 0.901x | 1.254x |
| perft_full_d3 | 0.996x | 0.989x | 1.049x |
| fen_parse | 0.990x | 0.982x | 1.231x |
| fen_write_native_ep | 1.001x | 0.998x | 1.582x |
| snapshot | 1.047x | 1.043x | 1.005x |
| san_parse_all | 1.000x | 0.985x | 1.823x |
| san_render_native | 0.997x | 0.985x | 2.528x |
| apply_children_full | 1.004x | 0.999x | 1.040x |
| make_unmake_full | 1.001x | 0.997x | 1.446x |
| startpos | 0.965x | 0.965x | 1.171x |
| chess960_materialize | 1.052x | 1.003x | 1.392x |
| chess960_perft_d3 | 1.019x | 0.968x | 1.359x |
| polyglot_key | 1.002x | 1.001x | 25.282x |
| polyglot_recompute | 0.997x | 0.997x | 1.071x |
| repetition_key | 1.006x | 1.000x | 18.779x |
| corpus_count | 0.818x | 0.788x | 1.218x |
| corpus_materialize | 0.973x | 0.966x | 1.173x |
| corpus_uci_sorted | 0.995x | 0.993x | 1.448x |
| replay_trusted_native | 1.007x | 1.005x | 0.499x |
| replay_checked_native | 0.992x | 0.991x | 0.982x |

Cold first legal move list: 0.010 ms -> 0.006 ms (1.89x).
