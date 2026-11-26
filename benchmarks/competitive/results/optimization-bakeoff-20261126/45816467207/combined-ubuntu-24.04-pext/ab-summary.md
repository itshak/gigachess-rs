| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.024x | 0.996x | 1.412x |
| materialize | 0.989x | 0.976x | 1.495x |
| perft_bulk_d4 | 0.968x | 0.954x | 1.038x |
| perft_full_d3 | 0.978x | 0.926x | 1.451x |
| fen_parse | 1.061x | 0.977x | 1.140x |
| fen_write_native_ep | 0.987x | 0.963x | 1.917x |
| snapshot | 0.993x | 0.978x | 1.152x |
| san_parse_all | 1.014x | 0.995x | 1.928x |
| san_render_native | 0.987x | 0.983x | 2.110x |
| apply_children_full | 0.993x | 0.990x | 1.182x |
| make_unmake_full | 0.991x | 0.983x | 1.123x |
| startpos | 0.999x | 0.999x | 1.112x |
| chess960_materialize | 1.002x | 0.998x | 1.424x |
| chess960_perft_d3 | 0.991x | 0.971x | 1.367x |
| polyglot_key | 0.987x | 0.980x | 39.568x |
| polyglot_recompute | 0.959x | 0.945x | 0.973x |
| repetition_key | 0.983x | 0.963x | 16.177x |
| corpus_count | 1.048x | 1.026x | 0.968x |
| corpus_materialize | 0.999x | 0.999x | 1.045x |
| corpus_uci_sorted | 0.985x | 0.983x | 1.543x |
| replay_trusted_native | 0.998x | 0.991x | 0.837x |
| replay_checked_native | 0.995x | 0.993x | 1.108x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.89x).
