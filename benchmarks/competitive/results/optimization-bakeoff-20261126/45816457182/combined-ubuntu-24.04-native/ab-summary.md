| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.998x | 0.955x | 1.372x |
| materialize | 1.001x | 0.980x | 1.510x |
| perft_bulk_d4 | 0.988x | 0.968x | 1.017x |
| perft_full_d3 | 0.995x | 0.986x | 1.403x |
| fen_parse | 1.008x | 0.997x | 1.161x |
| fen_write_native_ep | 1.033x | 0.998x | 1.907x |
| snapshot | 1.000x | 0.998x | 0.999x |
| san_parse_all | 1.110x | 1.084x | 1.852x |
| san_render_native | 0.999x | 0.971x | 2.207x |
| apply_children_full | 1.000x | 0.996x | 1.176x |
| make_unmake_full | 1.001x | 0.990x | 1.124x |
| startpos | 0.996x | 0.996x | 1.249x |
| chess960_materialize | 0.998x | 0.997x | 1.471x |
| chess960_perft_d3 | 0.993x | 0.993x | 1.408x |
| polyglot_key | 1.004x | 0.999x | 39.999x |
| polyglot_recompute | 1.004x | 1.002x | 1.023x |
| repetition_key | 1.014x | 1.000x | 18.871x |
| corpus_count | 1.018x | 1.008x | 0.872x |
| corpus_materialize | 1.006x | 1.002x | 1.030x |
| corpus_uci_sorted | 0.992x | 0.990x | 1.565x |
| replay_trusted_native | 0.963x | 0.944x | 0.789x |
| replay_checked_native | 0.991x | 0.989x | 1.085x |

Cold first legal move list: 0.005 ms -> 0.005 ms (1.00x).
