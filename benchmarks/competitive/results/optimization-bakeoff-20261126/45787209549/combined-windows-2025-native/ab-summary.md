| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.027x | 0.992x | 1.314x |
| materialize | 1.007x | 0.981x | 1.196x |
| perft_bulk_d4 | 1.010x | 0.983x | 1.339x |
| perft_full_d3 | 0.899x | 0.789x | 1.142x |
| fen_parse | 0.892x | 0.854x | 1.119x |
| fen_write_native_ep | 0.987x | 0.944x | 1.581x |
| snapshot | 1.321x | 1.309x | 0.927x |
| san_parse_all | 1.024x | 1.003x | 1.996x |
| san_render_native | 1.014x | 0.964x | 3.705x |
| apply_children_full | 0.986x | 0.972x | 1.191x |
| make_unmake_full | 0.970x | 0.944x | 1.236x |
| startpos | 1.004x | 1.004x | 1.107x |
| chess960_materialize | 0.992x | 0.985x | 1.225x |
| chess960_perft_d3 | 1.058x | 1.033x | 1.340x |
| polyglot_key | 0.943x | 0.875x | 27.507x |
| polyglot_recompute | 1.060x | 0.989x | 1.034x |
| repetition_key | 1.008x | 0.991x | 18.814x |
| corpus_count | 1.020x | 0.985x | 1.349x |
| corpus_materialize | 0.988x | 0.982x | 1.150x |
| corpus_uci_sorted | 1.011x | 1.007x | 1.402x |
| replay_trusted_native | 0.979x | 0.976x | 0.663x |
| replay_checked_native | 1.017x | 0.977x | 0.976x |

Cold first legal move list: 0.038 ms -> 0.040 ms (0.94x).
