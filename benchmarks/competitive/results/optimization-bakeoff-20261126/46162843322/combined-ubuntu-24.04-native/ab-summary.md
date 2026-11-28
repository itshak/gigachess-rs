| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.081x | 1.042x | 1.581x |
| materialize | 1.003x | 0.989x | 1.506x |
| perft_bulk_d4 | 1.056x | 1.021x | 1.230x |
| perft_full_d3 | 1.215x | 1.043x | 1.651x |
| fen_parse | 0.988x | 0.935x | 1.266x |
| fen_write_native_ep | 1.008x | 0.982x | 2.164x |
| snapshot | 0.989x | 0.971x | 1.139x |
| san_parse_all | 1.029x | 1.013x | 1.790x |
| san_render_native | 0.993x | 0.964x | 2.122x |
| apply_children_full | 1.006x | 0.992x | 1.345x |
| make_unmake_full | 0.948x | 0.929x | 1.139x |
| startpos | 1.007x | 1.007x | 1.141x |
| chess960_materialize | 0.992x | 0.934x | 1.363x |
| chess960_perft_d3 | 1.019x | 1.005x | 1.534x |
| polyglot_key | 1.003x | 1.003x | 41.573x |
| polyglot_recompute | 1.620x | 1.292x | 1.680x |
| repetition_key | 1.037x | 1.020x | 26.729x |
| corpus_count | 1.401x | 1.131x | 1.289x |
| corpus_materialize | 1.007x | 1.002x | 1.064x |
| corpus_uci_sorted | 1.001x | 0.995x | 1.560x |
| replay_trusted_native | 0.987x | 0.973x | 0.825x |
| replay_checked_native | 0.936x | 0.898x | 1.122x |

Cold first legal move list: 0.007 ms -> 0.008 ms (0.98x).
