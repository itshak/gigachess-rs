| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.954x | 0.921x | 1.136x |
| materialize | 1.003x | 0.970x | 1.160x |
| perft_bulk_d4 | 0.938x | 0.894x | 1.098x |
| perft_full_d3 | 0.947x | 0.905x | 1.079x |
| fen_parse | 1.179x | 1.153x | 1.265x |
| fen_write_native_ep | 0.912x | 0.863x | 1.598x |
| snapshot | 1.001x | 0.999x | 0.982x |
| san_parse_all | 0.989x | 0.969x | 1.967x |
| san_render_native | 0.798x | 0.717x | 1.876x |
| apply_children_full | 0.973x | 0.911x | 1.225x |
| make_unmake_full | 0.791x | 0.670x | 1.104x |
| startpos | 1.272x | 1.272x | 1.694x |
| chess960_materialize | 0.950x | 0.933x | 1.336x |
| chess960_perft_d3 | 0.905x | 0.867x | 1.340x |
| polyglot_key | 1.004x | 1.000x | 22.288x |
| polyglot_recompute | 0.994x | 0.981x | 0.984x |
| repetition_key | 0.995x | 0.981x | 20.446x |
| corpus_count | 0.943x | 0.918x | 1.078x |
| corpus_materialize | 0.965x | 0.946x | 1.181x |
| corpus_uci_sorted | 1.082x | 1.080x | 1.852x |
| replay_trusted_native | 1.002x | 0.998x | 0.951x |
| replay_checked_native | 0.935x | 0.912x | 1.011x |

Cold first legal move list: 0.010 ms -> 0.008 ms (1.21x).
