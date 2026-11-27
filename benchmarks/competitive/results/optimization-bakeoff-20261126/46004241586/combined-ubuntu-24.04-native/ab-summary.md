| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.054x | 1.002x | 1.571x |
| materialize | 0.948x | 0.912x | 1.530x |
| perft_bulk_d4 | 1.060x | 1.043x | 1.238x |
| perft_full_d3 | 1.230x | 1.054x | 1.659x |
| fen_parse | 0.992x | 0.949x | 1.174x |
| fen_write_native_ep | 1.016x | 0.992x | 1.871x |
| snapshot | 1.001x | 0.997x | 1.153x |
| san_parse_all | 1.003x | 0.993x | 1.875x |
| san_render_native | 1.001x | 0.997x | 2.108x |
| apply_children_full | 1.021x | 1.003x | 1.344x |
| make_unmake_full | 0.999x | 0.988x | 1.194x |
| startpos | 1.013x | 1.013x | 1.121x |
| chess960_materialize | 1.009x | 0.997x | 1.555x |
| chess960_perft_d3 | 1.021x | 1.002x | 1.529x |
| polyglot_key | 0.934x | 0.869x | 36.562x |
| polyglot_recompute | 1.025x | 1.015x | 1.019x |
| repetition_key | 0.988x | 0.979x | 19.193x |
| corpus_count | 1.212x | 1.122x | 1.308x |
| corpus_materialize | 1.010x | 1.005x | 1.052x |
| corpus_uci_sorted | 0.998x | 0.992x | 1.541x |
| replay_trusted_native | 1.016x | 0.997x | 0.890x |
| replay_checked_native | 0.982x | 0.982x | 1.161x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.12x).
