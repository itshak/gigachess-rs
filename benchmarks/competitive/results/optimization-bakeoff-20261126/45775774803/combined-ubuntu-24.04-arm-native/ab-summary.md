| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.293x |
| materialize | 1.001x | 0.999x | 1.210x |
| perft_bulk_d4 | 0.998x | 0.996x | 1.274x |
| perft_full_d3 | 0.995x | 0.986x | 0.748x |
| fen_parse | 0.989x | 0.986x | 1.218x |
| fen_write_native_ep | 0.996x | 0.989x | 1.036x |
| snapshot | 1.062x | 1.060x | 1.018x |
| san_parse_all | 1.011x | 0.991x | 1.767x |
| san_render_native | 1.004x | 0.997x | 2.445x |
| apply_children_full | 1.018x | 0.999x | 0.828x |
| make_unmake_full | 1.004x | 0.999x | 1.167x |
| startpos | 1.073x | 1.073x | 1.210x |
| chess960_materialize | 1.000x | 0.998x | 1.230x |
| chess960_perft_d3 | 1.008x | 1.008x | 1.224x |
| polyglot_key | 0.988x | 0.978x | 25.440x |
| polyglot_recompute | 1.001x | 1.000x | 1.071x |
| repetition_key | 0.988x | 0.974x | 18.861x |
| corpus_count | 1.000x | 0.997x | 1.418x |
| corpus_materialize | 1.000x | 0.999x | 1.191x |
| corpus_uci_sorted | 1.001x | 1.000x | 0.976x |
| replay_trusted_native | 1.002x | 1.002x | 0.412x |
| replay_checked_native | 1.019x | 1.011x | 0.925x |

Cold first legal move list: 0.012 ms -> 0.013 ms (0.98x).
