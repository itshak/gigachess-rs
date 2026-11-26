| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 0.987x | 1.380x |
| materialize | 1.015x | 0.989x | 1.307x |
| perft_bulk_d4 | 1.005x | 0.987x | 1.176x |
| perft_full_d3 | 1.047x | 1.022x | 0.857x |
| fen_parse | 1.005x | 0.980x | 1.303x |
| fen_write_native_ep | 1.008x | 0.970x | 1.334x |
| snapshot | 0.665x | 0.659x | 0.992x |
| san_parse_all | 1.040x | 1.004x | 1.638x |
| san_render_native | 1.139x | 0.984x | 3.981x |
| apply_children_full | 1.062x | 1.009x | 1.058x |
| make_unmake_full | 1.070x | 1.039x | 1.004x |
| startpos | 2.210x | 2.210x | 1.173x |
| chess960_materialize | 1.031x | 1.010x | 1.267x |
| chess960_perft_d3 | 1.016x | 1.016x | 1.342x |
| polyglot_key | 1.022x | 1.001x | 42.302x |
| polyglot_recompute | 0.992x | 0.973x | 0.958x |
| repetition_key | 1.050x | 1.017x | 23.730x |
| corpus_count | 0.970x | 0.952x | 1.271x |
| corpus_materialize | 1.046x | 1.002x | 1.097x |
| corpus_uci_sorted | 0.987x | 0.973x | 0.937x |
| replay_trusted_native | 1.081x | 1.075x | 0.612x |
| replay_checked_native | 1.080x | 1.038x | 0.989x |

Cold first legal move list: 0.039 ms -> 0.037 ms (1.06x).
