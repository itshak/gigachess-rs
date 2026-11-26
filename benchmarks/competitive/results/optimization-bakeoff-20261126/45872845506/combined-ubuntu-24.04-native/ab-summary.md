| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.996x | 0.989x | 1.484x |
| materialize | 0.964x | 0.952x | 1.526x |
| perft_bulk_d4 | 1.001x | 0.990x | 1.166x |
| perft_full_d3 | 0.994x | 0.987x | 1.342x |
| fen_parse | 0.981x | 0.961x | 1.132x |
| fen_write_native_ep | 1.004x | 0.970x | 2.057x |
| snapshot | 1.000x | 0.998x | 1.331x |
| san_parse_all | 1.002x | 0.992x | 1.923x |
| san_render_native | 1.011x | 0.989x | 2.199x |
| apply_children_full | 1.023x | 0.968x | 1.282x |
| make_unmake_full | 0.948x | 0.895x | 1.122x |
| startpos | 1.114x | 1.114x | 1.245x |
| chess960_materialize | 0.899x | 0.876x | 1.393x |
| chess960_perft_d3 | 0.998x | 0.989x | 1.499x |
| polyglot_key | 1.008x | 1.000x | 39.798x |
| polyglot_recompute | 1.020x | 1.017x | 1.031x |
| repetition_key | 0.976x | 0.965x | 19.256x |
| corpus_count | 1.000x | 0.998x | 1.098x |
| corpus_materialize | 1.000x | 0.996x | 1.053x |
| corpus_uci_sorted | 0.997x | 0.991x | 1.544x |
| replay_trusted_native | 0.964x | 0.935x | 0.850x |
| replay_checked_native | 0.984x | 0.982x | 1.171x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.31x).
