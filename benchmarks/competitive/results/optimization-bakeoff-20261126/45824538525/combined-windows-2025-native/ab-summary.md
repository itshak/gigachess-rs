| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.024x | 1.007x | 1.323x |
| materialize | 1.025x | 0.956x | 1.184x |
| perft_bulk_d4 | 0.989x | 0.961x | 1.185x |
| perft_full_d3 | 1.005x | 0.962x | 1.230x |
| fen_parse | 1.126x | 1.050x | 1.416x |
| fen_write_native_ep | 1.049x | 1.013x | 1.564x |
| snapshot | 1.027x | 0.977x | 0.963x |
| san_parse_all | 1.069x | 1.012x | 2.104x |
| san_render_native | 0.984x | 0.864x | 3.695x |
| apply_children_full | 1.021x | 0.995x | 1.380x |
| make_unmake_full | 1.008x | 0.979x | 1.313x |
| startpos | 1.040x | 1.040x | 1.250x |
| chess960_materialize | 1.083x | 1.026x | 1.285x |
| chess960_perft_d3 | 1.083x | 1.032x | 1.413x |
| polyglot_key | 0.934x | 0.825x | 26.311x |
| polyglot_recompute | 1.093x | 1.064x | 1.048x |
| repetition_key | 1.030x | 1.008x | 20.762x |
| corpus_count | 1.073x | 0.995x | 1.236x |
| corpus_materialize | 1.073x | 1.054x | 1.091x |
| corpus_uci_sorted | 0.941x | 0.904x | 1.320x |
| replay_trusted_native | 1.002x | 0.987x | 0.831x |
| replay_checked_native | 1.028x | 0.980x | 0.987x |

Cold first legal move list: 0.017 ms -> 0.017 ms (1.01x).
