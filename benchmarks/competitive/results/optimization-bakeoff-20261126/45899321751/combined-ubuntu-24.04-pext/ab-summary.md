| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.987x | 0.969x | 1.544x |
| materialize | 1.114x | 1.047x | 1.609x |
| perft_bulk_d4 | 1.033x | 1.018x | 1.232x |
| perft_full_d3 | 1.002x | 0.999x | 1.340x |
| fen_parse | 0.941x | 0.834x | 1.116x |
| fen_write_native_ep | 0.995x | 0.969x | 2.024x |
| snapshot | 1.000x | 0.984x | 1.155x |
| san_parse_all | 1.025x | 0.995x | 1.890x |
| san_render_native | 1.009x | 1.000x | 2.109x |
| apply_children_full | 0.999x | 0.992x | 1.338x |
| make_unmake_full | 1.003x | 0.985x | 1.189x |
| startpos | 1.130x | 1.130x | 1.120x |
| chess960_materialize | 1.008x | 0.985x | 1.505x |
| chess960_perft_d3 | 0.989x | 0.985x | 1.465x |
| polyglot_key | 0.995x | 0.994x | 39.386x |
| polyglot_recompute | 1.013x | 1.007x | 1.016x |
| repetition_key | 0.811x | 0.710x | 19.574x |
| corpus_count | 1.160x | 1.101x | 1.318x |
| corpus_materialize | 1.011x | 0.998x | 1.091x |
| corpus_uci_sorted | 1.006x | 1.004x | 1.553x |
| replay_trusted_native | 1.023x | 1.002x | 0.909x |
| replay_checked_native | 0.995x | 0.994x | 1.189x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.93x).
