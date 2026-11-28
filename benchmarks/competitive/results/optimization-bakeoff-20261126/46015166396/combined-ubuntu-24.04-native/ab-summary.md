| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.069x | 1.019x | 1.566x |
| materialize | 1.000x | 0.989x | 1.503x |
| perft_bulk_d4 | 1.073x | 1.043x | 1.236x |
| perft_full_d3 | 1.256x | 1.044x | 1.711x |
| fen_parse | 0.916x | 0.850x | 1.067x |
| fen_write_native_ep | 1.014x | 0.977x | 2.178x |
| snapshot | 1.000x | 0.998x | 1.330x |
| san_parse_all | 1.045x | 0.990x | 1.810x |
| san_render_native | 1.014x | 1.008x | 2.152x |
| apply_children_full | 0.997x | 0.983x | 1.247x |
| make_unmake_full | 0.978x | 0.926x | 1.174x |
| startpos | 1.117x | 1.117x | 1.250x |
| chess960_materialize | 1.049x | 0.998x | 1.426x |
| chess960_perft_d3 | 1.012x | 0.998x | 1.529x |
| polyglot_key | 1.009x | 0.999x | 41.984x |
| polyglot_recompute | 1.605x | 1.278x | 1.690x |
| repetition_key | 1.027x | 1.017x | 25.213x |
| corpus_count | 1.397x | 1.134x | 1.298x |
| corpus_materialize | 0.992x | 0.979x | 1.045x |
| corpus_uci_sorted | 1.013x | 1.010x | 1.570x |
| replay_trusted_native | 1.040x | 1.021x | 0.886x |
| replay_checked_native | 0.999x | 0.994x | 1.230x |

Cold first legal move list: 0.006 ms -> 0.007 ms (0.91x).
