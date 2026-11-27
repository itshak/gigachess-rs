| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 0.971x | 1.577x |
| materialize | 0.997x | 0.969x | 1.363x |
| perft_bulk_d4 | 1.048x | 1.005x | 1.224x |
| perft_full_d3 | 1.207x | 1.037x | 1.662x |
| fen_parse | 0.878x | 0.859x | 1.063x |
| fen_write_native_ep | 0.996x | 0.982x | 1.708x |
| snapshot | 1.126x | 1.058x | 1.261x |
| san_parse_all | 0.959x | 0.937x | 1.680x |
| san_render_native | 1.009x | 0.999x | 4.141x |
| apply_children_full | 1.001x | 0.985x | 1.274x |
| make_unmake_full | 0.994x | 0.965x | 1.206x |
| startpos | 1.004x | 1.004x | 0.997x |
| chess960_materialize | 0.993x | 0.981x | 1.339x |
| chess960_perft_d3 | 1.019x | 0.999x | 1.527x |
| polyglot_key | 1.006x | 1.002x | 41.520x |
| polyglot_recompute | 0.986x | 0.984x | 0.999x |
| repetition_key | 1.070x | 0.875x | 27.729x |
| corpus_count | 1.191x | 1.119x | 1.161x |
| corpus_materialize | 1.015x | 1.009x | 1.077x |
| corpus_uci_sorted | 1.004x | 0.997x | 1.630x |
| replay_trusted_native | 0.913x | 0.871x | 0.788x |
| replay_checked_native | 1.002x | 0.995x | 1.173x |

Cold first legal move list: 0.022 ms -> 0.017 ms (1.30x).
