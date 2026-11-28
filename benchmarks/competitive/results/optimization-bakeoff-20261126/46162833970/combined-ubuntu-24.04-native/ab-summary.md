| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 0.930x | 1.496x |
| materialize | 1.003x | 0.998x | 1.171x |
| perft_bulk_d4 | 1.010x | 0.954x | 1.343x |
| perft_full_d3 | 1.339x | 1.076x | 1.601x |
| fen_parse | 0.850x | 0.811x | 0.984x |
| fen_write_native_ep | 1.009x | 0.990x | 1.819x |
| snapshot | 1.002x | 0.997x | 1.249x |
| san_parse_all | 1.007x | 0.993x | 2.187x |
| san_render_native | 1.006x | 0.993x | 2.114x |
| apply_children_full | 1.005x | 0.990x | 1.323x |
| make_unmake_full | 1.033x | 0.969x | 1.276x |
| startpos | 0.999x | 0.999x | 1.197x |
| chess960_materialize | 1.008x | 1.005x | 1.561x |
| chess960_perft_d3 | 1.015x | 0.999x | 1.611x |
| polyglot_key | 0.992x | 0.985x | 20.592x |
| polyglot_recompute | 1.236x | 1.066x | 1.264x |
| repetition_key | 0.993x | 0.990x | 18.920x |
| corpus_count | 1.057x | 1.029x | 1.375x |
| corpus_materialize | 1.010x | 1.009x | 1.248x |
| corpus_uci_sorted | 0.996x | 0.996x | 1.694x |
| replay_trusted_native | 1.009x | 1.003x | 1.145x |
| replay_checked_native | 1.009x | 1.007x | 1.142x |

Cold first legal move list: 0.007 ms -> 0.006 ms (1.13x).
