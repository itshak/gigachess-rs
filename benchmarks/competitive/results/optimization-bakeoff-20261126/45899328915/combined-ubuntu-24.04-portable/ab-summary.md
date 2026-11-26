| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.914x | 0.861x | 1.184x |
| materialize | 0.946x | 0.927x | 1.627x |
| perft_bulk_d4 | 0.978x | 0.887x | 1.161x |
| perft_full_d3 | 1.049x | 0.953x | 1.383x |
| fen_parse | 1.118x | 1.056x | 1.179x |
| fen_write_native_ep | 0.929x | 0.875x | 1.665x |
| snapshot | 0.894x | 0.892x | 1.001x |
| san_parse_all | 0.987x | 0.970x | 1.877x |
| san_render_native | 1.012x | 0.965x | 2.206x |
| apply_children_full | 0.970x | 0.872x | 1.343x |
| make_unmake_full | 0.988x | 0.897x | 1.184x |
| startpos | 1.166x | 1.166x | 1.500x |
| chess960_materialize | 0.846x | 0.830x | 1.313x |
| chess960_perft_d3 | 0.957x | 0.918x | 1.438x |
| polyglot_key | 1.027x | 1.000x | 41.249x |
| polyglot_recompute | 1.003x | 0.992x | 0.965x |
| repetition_key | 1.112x | 0.935x | 23.015x |
| corpus_count | 1.076x | 1.029x | 1.216x |
| corpus_materialize | 0.982x | 0.974x | 1.124x |
| corpus_uci_sorted | 0.990x | 0.986x | 1.550x |
| replay_trusted_native | 0.907x | 0.880x | 0.779x |
| replay_checked_native | 0.893x | 0.848x | 0.998x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.78x).
