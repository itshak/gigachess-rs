| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.009x | 1.000x | 1.185x |
| materialize | 1.001x | 0.988x | 1.622x |
| perft_bulk_d4 | 0.995x | 0.985x | 1.153x |
| perft_full_d3 | 1.012x | 0.998x | 1.483x |
| fen_parse | 0.968x | 0.929x | 1.152x |
| fen_write_native_ep | 1.004x | 0.983x | 1.198x |
| snapshot | 0.801x | 0.795x | 0.995x |
| san_parse_all | 1.002x | 0.987x | 1.896x |
| san_render_native | 1.020x | 1.012x | 2.169x |
| apply_children_full | 1.093x | 1.047x | 1.241x |
| make_unmake_full | 1.001x | 0.953x | 1.171x |
| startpos | 1.002x | 1.002x | 1.283x |
| chess960_materialize | 1.076x | 1.067x | 1.424x |
| chess960_perft_d3 | 1.001x | 0.998x | 1.330x |
| polyglot_key | 0.944x | 0.842x | 37.519x |
| polyglot_recompute | 0.933x | 0.905x | 0.947x |
| repetition_key | 0.883x | 0.784x | 19.606x |
| corpus_count | 1.015x | 1.014x | 1.275x |
| corpus_materialize | 1.001x | 0.999x | 1.157x |
| corpus_uci_sorted | 1.015x | 1.012x | 1.552x |
| replay_trusted_native | 1.052x | 0.961x | 0.723x |
| replay_checked_native | 0.917x | 0.876x | 1.010x |

Cold first legal move list: 0.010 ms -> 0.009 ms (1.14x).
