| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.990x | 0.958x | 1.277x |
| materialize | 0.987x | 0.956x | 1.339x |
| perft_bulk_d4 | 1.004x | 0.980x | 1.305x |
| perft_full_d3 | 1.005x | 0.998x | 1.312x |
| fen_parse | 1.180x | 1.109x | 1.290x |
| fen_write_native_ep | 1.033x | 1.008x | 1.671x |
| snapshot | 0.998x | 0.993x | 0.966x |
| san_parse_all | 0.987x | 0.971x | 2.016x |
| san_render_native | 0.994x | 0.984x | 2.234x |
| apply_children_full | 0.880x | 0.833x | 1.247x |
| make_unmake_full | 0.881x | 0.841x | 1.298x |
| startpos | 1.000x | 1.000x | 1.351x |
| chess960_materialize | 0.992x | 0.982x | 1.473x |
| chess960_perft_d3 | 0.999x | 0.988x | 1.575x |
| polyglot_key | 0.999x | 0.997x | 28.126x |
| polyglot_recompute | 1.013x | 1.013x | 1.045x |
| repetition_key | 0.987x | 0.978x | 19.823x |
| corpus_count | 0.984x | 0.976x | 1.284x |
| corpus_materialize | 1.006x | 1.004x | 1.233x |
| corpus_uci_sorted | 1.002x | 0.994x | 1.800x |
| replay_trusted_native | 0.844x | 0.832x | 0.777x |
| replay_checked_native | 1.008x | 1.008x | 1.159x |

Cold first legal move list: 0.008 ms -> 0.009 ms (0.90x).
