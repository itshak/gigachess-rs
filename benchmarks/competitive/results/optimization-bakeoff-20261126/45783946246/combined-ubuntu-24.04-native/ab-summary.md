| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.997x | 0.991x | 1.399x |
| materialize | 1.038x | 0.973x | 1.532x |
| perft_bulk_d4 | 1.001x | 0.998x | 1.212x |
| perft_full_d3 | 1.100x | 1.035x | 1.468x |
| fen_parse | 1.052x | 1.020x | 1.023x |
| fen_write_native_ep | 1.003x | 0.995x | 1.924x |
| snapshot | 1.096x | 1.093x | 1.152x |
| san_parse_all | 0.940x | 0.921x | 1.770x |
| san_render_native | 0.974x | 0.969x | 2.070x |
| apply_children_full | 1.296x | 1.231x | 1.089x |
| make_unmake_full | 0.986x | 0.968x | 1.153x |
| startpos | 1.000x | 1.000x | 1.206x |
| chess960_materialize | 0.992x | 0.946x | 1.428x |
| chess960_perft_d3 | 0.999x | 0.995x | 1.375x |
| polyglot_key | 0.989x | 0.977x | 39.598x |
| polyglot_recompute | 1.030x | 1.019x | 1.005x |
| repetition_key | 1.174x | 0.976x | 22.384x |
| corpus_count | 0.997x | 0.993x | 1.341x |
| corpus_materialize | 1.031x | 1.030x | 1.115x |
| corpus_uci_sorted | 1.005x | 1.001x | 1.544x |
| replay_trusted_native | 0.986x | 0.984x | 0.714x |
| replay_checked_native | 1.020x | 1.017x | 1.087x |

Cold first legal move list: 0.016 ms -> 0.009 ms (1.70x).
