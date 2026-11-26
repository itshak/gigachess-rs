| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.014x | 0.978x | 1.404x |
| materialize | 0.995x | 0.991x | 1.501x |
| perft_bulk_d4 | 0.974x | 0.917x | 1.038x |
| perft_full_d3 | 0.936x | 0.884x | 1.345x |
| fen_parse | 1.058x | 0.973x | 1.093x |
| fen_write_native_ep | 0.990x | 0.981x | 1.854x |
| snapshot | 1.000x | 0.996x | 1.154x |
| san_parse_all | 1.040x | 1.021x | 1.955x |
| san_render_native | 0.984x | 0.965x | 2.107x |
| apply_children_full | 0.914x | 0.848x | 1.194x |
| make_unmake_full | 0.875x | 0.826x | 1.021x |
| startpos | 0.890x | 0.890x | 1.119x |
| chess960_materialize | 0.938x | 0.895x | 1.350x |
| chess960_perft_d3 | 0.953x | 0.946x | 1.326x |
| polyglot_key | 1.001x | 1.000x | 39.125x |
| polyglot_recompute | 0.981x | 0.977x | 0.997x |
| repetition_key | 1.010x | 0.997x | 19.123x |
| corpus_count | 1.034x | 1.023x | 0.957x |
| corpus_materialize | 0.995x | 0.992x | 1.066x |
| corpus_uci_sorted | 0.996x | 0.993x | 1.555x |
| replay_trusted_native | 0.856x | 0.783x | 0.775x |
| replay_checked_native | 0.999x | 0.993x | 1.030x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.11x).
