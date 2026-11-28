| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.039x | 1.008x | 1.623x |
| materialize | 1.098x | 1.037x | 1.604x |
| perft_bulk_d4 | 1.064x | 1.042x | 1.274x |
| perft_full_d3 | 1.202x | 1.053x | 1.634x |
| fen_parse | 1.019x | 0.986x | 1.150x |
| fen_write_native_ep | 0.997x | 0.987x | 2.095x |
| snapshot | 1.002x | 0.997x | 1.331x |
| san_parse_all | 1.102x | 1.076x | 1.854x |
| san_render_native | 1.002x | 0.999x | 2.218x |
| apply_children_full | 1.002x | 0.992x | 1.264x |
| make_unmake_full | 0.972x | 0.948x | 1.159x |
| startpos | 1.000x | 1.000x | 1.251x |
| chess960_materialize | 1.007x | 0.996x | 1.537x |
| chess960_perft_d3 | 1.004x | 0.983x | 1.476x |
| polyglot_key | 1.008x | 0.999x | 40.158x |
| polyglot_recompute | 1.660x | 1.344x | 1.629x |
| repetition_key | 1.000x | 0.999x | 19.373x |
| corpus_count | 1.222x | 1.121x | 1.264x |
| corpus_materialize | 0.997x | 0.993x | 1.083x |
| corpus_uci_sorted | 0.991x | 0.989x | 1.556x |
| replay_trusted_native | 1.039x | 1.008x | 0.916x |
| replay_checked_native | 0.998x | 0.996x | 1.181x |

Cold first legal move list: 0.007 ms -> 0.005 ms (1.49x).
