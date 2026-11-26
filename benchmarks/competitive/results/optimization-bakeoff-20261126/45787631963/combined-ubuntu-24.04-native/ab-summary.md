| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.973x | 0.915x | 1.367x |
| materialize | 0.986x | 0.907x | 1.478x |
| perft_bulk_d4 | 0.849x | 0.768x | 1.017x |
| perft_full_d3 | 0.982x | 0.908x | 1.440x |
| fen_parse | 1.065x | 1.042x | 1.089x |
| fen_write_native_ep | 0.963x | 0.914x | 2.059x |
| snapshot | 2.358x | 2.309x | 1.148x |
| san_parse_all | 0.986x | 0.968x | 1.901x |
| san_render_native | 0.996x | 0.991x | 2.157x |
| apply_children_full | 1.000x | 0.999x | 1.092x |
| make_unmake_full | 1.018x | 1.001x | 1.136x |
| startpos | 1.000x | 1.000x | 1.206x |
| chess960_materialize | 0.987x | 0.922x | 1.339x |
| chess960_perft_d3 | 1.005x | 0.975x | 1.396x |
| polyglot_key | 1.058x | 1.000x | 39.254x |
| polyglot_recompute | 0.997x | 0.996x | 1.051x |
| repetition_key | 1.005x | 0.986x | 21.157x |
| corpus_count | 0.655x | 0.552x | 0.867x |
| corpus_materialize | 0.955x | 0.941x | 1.038x |
| corpus_uci_sorted | 1.006x | 1.005x | 1.574x |
| replay_trusted_native | 1.006x | 0.995x | 0.726x |
| replay_checked_native | 0.992x | 0.988x | 1.117x |

Cold first legal move list: 0.009 ms -> 0.006 ms (1.49x).
