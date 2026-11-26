| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.942x | 0.878x | 1.202x |
| materialize | 0.955x | 0.915x | 1.642x |
| perft_bulk_d4 | 0.955x | 0.916x | 1.131x |
| perft_full_d3 | 1.182x | 1.064x | 1.575x |
| fen_parse | 1.092x | 1.029x | 1.130x |
| fen_write_native_ep | 1.015x | 0.988x | 1.814x |
| snapshot | 1.002x | 0.994x | 1.003x |
| san_parse_all | 1.022x | 1.007x | 1.932x |
| san_render_native | 0.973x | 0.951x | 2.086x |
| apply_children_full | 1.003x | 0.998x | 1.346x |
| make_unmake_full | 1.047x | 0.993x | 1.245x |
| startpos | 0.997x | 0.997x | 1.282x |
| chess960_materialize | 0.917x | 0.905x | 1.424x |
| chess960_perft_d3 | 0.983x | 0.977x | 1.477x |
| polyglot_key | 1.024x | 0.993x | 40.541x |
| polyglot_recompute | 1.003x | 0.997x | 0.946x |
| repetition_key | 1.232x | 1.112x | 24.523x |
| corpus_count | 0.886x | 0.871x | 0.975x |
| corpus_materialize | 0.953x | 0.939x | 1.094x |
| corpus_uci_sorted | 0.978x | 0.970x | 1.520x |
| replay_trusted_native | 1.002x | 0.996x | 0.874x |
| replay_checked_native | 0.999x | 0.990x | 1.191x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.08x).
