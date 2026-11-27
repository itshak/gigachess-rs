| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.995x | 0.786x | 1.438x |
| materialize | 0.975x | 0.757x | 1.000x |
| perft_bulk_d4 | 0.973x | 0.704x | 1.289x |
| perft_full_d3 | 1.071x | 0.879x | 1.358x |
| fen_parse | 1.027x | 0.741x | 1.232x |
| fen_write_native_ep | 0.996x | 0.794x | 1.662x |
| snapshot | 1.068x | 0.954x | 0.987x |
| san_parse_all | 1.079x | 0.878x | 1.940x |
| san_render_native | 1.018x | 0.852x | 2.897x |
| apply_children_full | 1.076x | 0.802x | 1.151x |
| make_unmake_full | 1.071x | 0.887x | 1.356x |
| startpos | 1.022x | 1.022x | 1.177x |
| chess960_materialize | 1.102x | 1.000x | 1.519x |
| chess960_perft_d3 | 1.055x | 1.015x | 1.579x |
| polyglot_key | 1.020x | 0.965x | 39.472x |
| polyglot_recompute | 1.054x | 1.014x | 1.015x |
| repetition_key | 0.989x | 0.961x | 20.907x |
| corpus_count | 1.086x | 0.977x | 1.114x |
| corpus_materialize | 1.157x | 1.088x | 1.089x |
| corpus_uci_sorted | 1.028x | 1.019x | 1.312x |
| replay_trusted_native | 1.136x | 1.064x | 0.664x |
| replay_checked_native | 1.046x | 0.997x | 1.288x |

Cold first legal move list: 0.004 ms -> 0.004 ms (1.01x).
