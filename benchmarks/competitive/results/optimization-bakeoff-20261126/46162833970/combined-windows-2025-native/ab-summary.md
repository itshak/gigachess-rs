| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.040x | 1.006x | 1.599x |
| materialize | 0.986x | 0.927x | 1.381x |
| perft_bulk_d4 | 1.070x | 1.023x | 1.258x |
| perft_full_d3 | 1.212x | 1.049x | 1.660x |
| fen_parse | 1.131x | 1.062x | 1.153x |
| fen_write_native_ep | 1.004x | 0.989x | 1.680x |
| snapshot | 0.907x | 0.878x | 1.138x |
| san_parse_all | 1.082x | 1.053x | 1.765x |
| san_render_native | 1.000x | 0.981x | 4.072x |
| apply_children_full | 1.007x | 0.980x | 1.339x |
| make_unmake_full | 1.024x | 0.981x | 1.209x |
| startpos | 0.983x | 0.983x | 0.987x |
| chess960_materialize | 0.981x | 0.957x | 1.358x |
| chess960_perft_d3 | 1.018x | 1.005x | 1.519x |
| polyglot_key | 1.015x | 1.000x | 41.167x |
| polyglot_recompute | 1.569x | 1.209x | 1.601x |
| repetition_key | 0.997x | 0.988x | 33.597x |
| corpus_count | 1.189x | 1.119x | 1.128x |
| corpus_materialize | 1.048x | 1.014x | 1.091x |
| corpus_uci_sorted | 1.047x | 0.998x | 1.550x |
| replay_trusted_native | 1.056x | 1.030x | 0.851x |
| replay_checked_native | 0.987x | 0.974x | 1.174x |

Cold first legal move list: 0.015 ms -> 0.016 ms (0.90x).
