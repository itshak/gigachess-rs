| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 1.011x | 1.298x |
| materialize | 0.990x | 0.983x | 1.200x |
| perft_bulk_d4 | 0.990x | 0.975x | 1.261x |
| perft_full_d3 | 1.034x | 1.006x | 1.214x |
| fen_parse | 1.025x | 0.991x | 1.117x |
| fen_write_native_ep | 1.005x | 0.997x | 1.685x |
| snapshot | 0.756x | 0.755x | 1.037x |
| san_parse_all | 0.958x | 0.942x | 2.000x |
| san_render_native | 0.900x | 0.838x | 2.053x |
| apply_children_full | 0.996x | 0.974x | 1.150x |
| make_unmake_full | 0.983x | 0.981x | 1.217x |
| startpos | 0.927x | 0.927x | 1.195x |
| chess960_materialize | 1.006x | 1.002x | 1.304x |
| chess960_perft_d3 | 0.990x | 0.987x | 1.330x |
| polyglot_key | 1.083x | 0.995x | 27.427x |
| polyglot_recompute | 1.001x | 1.001x | 1.011x |
| repetition_key | 1.030x | 0.974x | 20.416x |
| corpus_count | 1.002x | 1.000x | 1.356x |
| corpus_materialize | 1.005x | 1.004x | 1.211x |
| corpus_uci_sorted | 1.009x | 1.003x | 1.799x |
| replay_trusted_native | 1.021x | 1.003x | 0.715x |
| replay_checked_native | 0.935x | 0.880x | 0.913x |

Cold first legal move list: 0.014 ms -> 0.013 ms (1.11x).
