| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.379x |
| materialize | 1.013x | 0.999x | 1.254x |
| perft_bulk_d4 | 0.995x | 0.990x | 1.226x |
| perft_full_d3 | 0.998x | 0.973x | 1.062x |
| fen_parse | 1.001x | 0.996x | 1.240x |
| fen_write_native_ep | 0.990x | 0.978x | 1.578x |
| snapshot | 0.991x | 0.984x | 0.960x |
| san_parse_all | 1.013x | 1.001x | 1.853x |
| san_render_native | 1.022x | 0.999x | 2.569x |
| apply_children_full | 0.955x | 0.918x | 1.239x |
| make_unmake_full | 1.011x | 0.970x | 1.528x |
| startpos | 1.000x | 1.000x | 1.334x |
| chess960_materialize | 1.008x | 0.997x | 1.394x |
| chess960_perft_d3 | 0.991x | 0.991x | 1.348x |
| polyglot_key | 1.000x | 0.998x | 25.793x |
| polyglot_recompute | 1.024x | 1.019x | 1.091x |
| repetition_key | 0.992x | 0.985x | 18.288x |
| corpus_count | 1.004x | 0.998x | 1.208x |
| corpus_materialize | 1.006x | 1.001x | 1.170x |
| corpus_uci_sorted | 0.995x | 0.993x | 1.458x |
| replay_trusted_native | 1.026x | 1.025x | 0.634x |
| replay_checked_native | 1.008x | 1.006x | 1.075x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.02x).
