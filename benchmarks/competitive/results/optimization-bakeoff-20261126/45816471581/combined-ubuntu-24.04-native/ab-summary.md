| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.003x | 0.992x | 1.322x |
| materialize | 0.997x | 0.990x | 1.324x |
| perft_bulk_d4 | 1.009x | 0.997x | 1.070x |
| perft_full_d3 | 0.996x | 0.983x | 1.271x |
| fen_parse | 1.087x | 1.053x | 1.228x |
| fen_write_native_ep | 1.018x | 0.997x | 1.844x |
| snapshot | 1.000x | 0.998x | 1.229x |
| san_parse_all | 0.962x | 0.924x | 1.856x |
| san_render_native | 1.005x | 0.996x | 2.428x |
| apply_children_full | 0.989x | 0.956x | 1.121x |
| make_unmake_full | 0.982x | 0.930x | 1.209x |
| startpos | 1.002x | 1.002x | 1.118x |
| chess960_materialize | 0.983x | 0.973x | 1.386x |
| chess960_perft_d3 | 1.000x | 0.995x | 1.398x |
| polyglot_key | 0.973x | 0.951x | 29.884x |
| polyglot_recompute | 0.993x | 0.990x | 1.106x |
| repetition_key | 0.997x | 0.995x | 15.222x |
| corpus_count | 0.999x | 0.998x | 0.901x |
| corpus_materialize | 0.991x | 0.990x | 1.042x |
| corpus_uci_sorted | 0.995x | 0.991x | 1.502x |
| replay_trusted_native | 0.988x | 0.977x | 0.754x |
| replay_checked_native | 0.973x | 0.968x | 1.052x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.99x).
