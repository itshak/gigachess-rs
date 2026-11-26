| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.021x | 0.966x | 1.235x |
| materialize | 1.032x | 0.956x | 1.148x |
| perft_bulk_d4 | 1.020x | 1.003x | 1.312x |
| perft_full_d3 | 0.990x | 0.957x | 0.719x |
| fen_parse | 0.987x | 0.921x | 1.209x |
| fen_write_native_ep | 1.008x | 0.991x | 1.011x |
| snapshot | 1.006x | 0.967x | 0.913x |
| san_parse_all | 1.041x | 1.007x | 1.802x |
| san_render_native | 1.011x | 0.972x | 3.515x |
| apply_children_full | 1.081x | 0.978x | 0.830x |
| make_unmake_full | 0.982x | 0.949x | 0.959x |
| startpos | 1.079x | 1.079x | 1.193x |
| chess960_materialize | 1.011x | 0.942x | 1.150x |
| chess960_perft_d3 | 0.960x | 0.940x | 1.221x |
| polyglot_key | 1.047x | 1.001x | 25.295x |
| polyglot_recompute | 1.040x | 1.008x | 1.050x |
| repetition_key | 1.121x | 1.100x | 21.372x |
| corpus_count | 1.005x | 0.992x | 1.351x |
| corpus_materialize | 0.980x | 0.970x | 1.148x |
| corpus_uci_sorted | 1.002x | 0.990x | 0.965x |
| replay_trusted_native | 1.102x | 1.079x | 0.562x |
| replay_checked_native | 1.003x | 0.987x | 0.858x |

Cold first legal move list: 0.042 ms -> 0.040 ms (1.03x).
