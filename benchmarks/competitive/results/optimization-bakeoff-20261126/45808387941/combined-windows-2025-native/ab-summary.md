| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.031x | 0.846x | 1.289x |
| materialize | 1.002x | 0.929x | 1.146x |
| perft_bulk_d4 | 0.905x | 0.813x | 1.184x |
| perft_full_d3 | 1.001x | 0.918x | 1.216x |
| fen_parse | 0.925x | 0.892x | 1.155x |
| fen_write_native_ep | 0.997x | 0.976x | 1.574x |
| snapshot | 0.988x | 0.869x | 0.976x |
| san_parse_all | 1.002x | 0.894x | 2.059x |
| san_render_native | 0.973x | 0.913x | 3.837x |
| apply_children_full | 1.125x | 0.769x | 1.249x |
| make_unmake_full | 0.962x | 0.830x | 1.246x |
| startpos | 1.477x | 1.477x | 1.248x |
| chess960_materialize | 1.060x | 1.020x | 1.268x |
| chess960_perft_d3 | 1.203x | 1.163x | 1.414x |
| polyglot_key | 0.998x | 0.859x | 27.813x |
| polyglot_recompute | 1.095x | 1.085x | 1.095x |
| repetition_key | 0.980x | 0.861x | 19.094x |
| corpus_count | 0.991x | 0.897x | 1.249x |
| corpus_materialize | 0.940x | 0.893x | 1.073x |
| corpus_uci_sorted | 1.012x | 0.979x | 1.392x |
| replay_trusted_native | 1.042x | 1.003x | 0.740x |
| replay_checked_native | 1.013x | 1.004x | 0.979x |

Cold first legal move list: 0.045 ms -> 0.023 ms (1.97x).
