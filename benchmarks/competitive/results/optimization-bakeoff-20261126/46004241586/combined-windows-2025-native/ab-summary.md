| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.035x | 0.968x | 1.549x |
| materialize | 1.004x | 0.973x | 1.283x |
| perft_bulk_d4 | 1.032x | 0.987x | 1.423x |
| perft_full_d3 | 1.197x | 1.012x | 1.579x |
| fen_parse | 1.053x | 1.018x | 1.331x |
| fen_write_native_ep | 0.958x | 0.882x | 1.546x |
| snapshot | 0.994x | 0.942x | 0.993x |
| san_parse_all | 0.994x | 0.915x | 2.030x |
| san_render_native | 1.010x | 0.975x | 4.055x |
| apply_children_full | 1.008x | 0.990x | 1.442x |
| make_unmake_full | 0.994x | 0.953x | 1.339x |
| startpos | 1.085x | 1.085x | 1.247x |
| chess960_materialize | 1.037x | 1.033x | 1.476x |
| chess960_perft_d3 | 0.978x | 0.973x | 1.518x |
| polyglot_key | 0.997x | 0.995x | 30.036x |
| polyglot_recompute | 0.968x | 0.947x | 0.990x |
| repetition_key | 1.003x | 0.966x | 21.034x |
| corpus_count | 1.092x | 1.089x | 1.469x |
| corpus_materialize | 0.990x | 0.953x | 1.134x |
| corpus_uci_sorted | 1.057x | 1.019x | 1.433x |
| replay_trusted_native | 1.015x | 0.967x | 0.836x |
| replay_checked_native | 0.993x | 0.982x | 1.089x |

Cold first legal move list: 0.016 ms -> 0.014 ms (1.10x).
