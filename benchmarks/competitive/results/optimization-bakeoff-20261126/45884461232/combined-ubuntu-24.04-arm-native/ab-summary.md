| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.000x | 0.999x | 1.552x |
| materialize | 0.999x | 0.997x | 1.320x |
| perft_bulk_d4 | 1.001x | 0.998x | 1.399x |
| perft_full_d3 | 1.000x | 0.999x | 1.335x |
| fen_parse | 0.998x | 0.994x | 1.218x |
| fen_write_native_ep | 0.998x | 0.997x | 1.569x |
| snapshot | 1.001x | 0.996x | 0.947x |
| san_parse_all | 0.998x | 0.996x | 1.824x |
| san_render_native | 1.002x | 0.993x | 2.684x |
| apply_children_full | 0.977x | 0.946x | 1.129x |
| make_unmake_full | 0.924x | 0.908x | 1.442x |
| startpos | 1.000x | 1.000x | 1.304x |
| chess960_materialize | 1.002x | 0.999x | 1.497x |
| chess960_perft_d3 | 1.003x | 1.001x | 1.502x |
| polyglot_key | 1.008x | 1.002x | 25.424x |
| polyglot_recompute | 1.001x | 1.000x | 1.088x |
| repetition_key | 1.003x | 0.999x | 18.927x |
| corpus_count | 0.994x | 0.992x | 1.403x |
| corpus_materialize | 0.997x | 0.994x | 1.238x |
| corpus_uci_sorted | 0.997x | 0.995x | 1.465x |
| replay_trusted_native | 0.916x | 0.873x | 0.568x |
| replay_checked_native | 0.930x | 0.916x | 1.058x |

Cold first legal move list: 0.006 ms -> 0.006 ms (1.01x).
