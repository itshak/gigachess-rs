| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.002x | 0.977x | 1.232x |
| materialize | 1.002x | 0.956x | 1.336x |
| perft_bulk_d4 | 1.009x | 0.971x | 1.238x |
| perft_full_d3 | 1.009x | 0.963x | 1.336x |
| fen_parse | 1.018x | 0.955x | 1.164x |
| fen_write_native_ep | 0.995x | 0.950x | 1.490x |
| snapshot | 0.986x | 0.926x | 1.000x |
| san_parse_all | 0.984x | 0.935x | 1.734x |
| san_render_native | 0.981x | 0.920x | 2.307x |
| apply_children_full | 1.004x | 0.970x | 1.290x |
| make_unmake_full | 0.986x | 0.945x | 1.340x |
| startpos | 0.976x | 0.976x | 1.288x |
| chess960_materialize | 0.949x | 0.944x | 1.546x |
| chess960_perft_d3 | 0.965x | 0.960x | 1.507x |
| polyglot_key | 1.008x | 0.975x | 31.786x |
| polyglot_recompute | 1.037x | 0.798x | 1.042x |
| repetition_key | 1.003x | 0.994x | 20.576x |
| corpus_count | 0.997x | 0.996x | 1.078x |
| corpus_materialize | 0.964x | 0.929x | 1.185x |
| corpus_uci_sorted | 0.987x | 0.984x | 1.680x |
| replay_trusted_native | 0.959x | 0.955x | 0.725x |
| replay_checked_native | 1.036x | 1.022x | 1.241x |

Cold first legal move list: 0.006 ms -> 0.006 ms (0.93x).
