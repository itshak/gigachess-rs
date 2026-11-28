| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.036x | 0.991x | 1.627x |
| materialize | 1.064x | 1.026x | 1.642x |
| perft_bulk_d4 | 1.056x | 1.023x | 1.268x |
| perft_full_d3 | 1.239x | 1.057x | 1.670x |
| fen_parse | 0.943x | 0.814x | 1.080x |
| fen_write_native_ep | 1.002x | 0.996x | 1.948x |
| snapshot | 0.999x | 0.985x | 1.000x |
| san_parse_all | 1.121x | 1.098x | 1.883x |
| san_render_native | 1.012x | 1.007x | 2.174x |
| apply_children_full | 1.020x | 1.003x | 1.356x |
| make_unmake_full | 0.987x | 0.953x | 1.174x |
| startpos | 1.114x | 1.114x | 1.248x |
| chess960_materialize | 1.008x | 0.995x | 1.512x |
| chess960_perft_d3 | 1.010x | 0.989x | 1.496x |
| polyglot_key | 0.908x | 0.824x | 35.916x |
| polyglot_recompute | 1.672x | 1.355x | 1.635x |
| repetition_key | 0.942x | 0.885x | 18.229x |
| corpus_count | 1.226x | 1.123x | 1.319x |
| corpus_materialize | 0.996x | 0.993x | 1.082x |
| corpus_uci_sorted | 0.991x | 0.986x | 1.557x |
| replay_trusted_native | 1.003x | 1.000x | 0.860x |
| replay_checked_native | 0.985x | 0.985x | 1.114x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.80x).
