| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.955x | 0.874x | 1.188x |
| materialize | 0.988x | 0.930x | 1.073x |
| perft_bulk_d4 | 0.996x | 0.986x | 0.976x |
| perft_full_d3 | 0.983x | 0.935x | 0.616x |
| fen_parse | 0.976x | 0.943x | 1.047x |
| fen_write_native_ep | 1.009x | 0.991x | 1.267x |
| snapshot | 0.978x | 0.975x | 0.858x |
| san_parse_all | 0.995x | 0.985x | 0.980x |
| san_render_native | 1.609x | 1.133x | 1.867x |
| apply_children_full | 0.991x | 0.983x | 0.882x |
| make_unmake_full | 1.002x | 0.997x | 0.839x |
| startpos | 1.058x | 1.058x | 0.872x |
| chess960_materialize | 1.017x | 1.011x | 1.202x |
| chess960_perft_d3 | 0.988x | 0.984x | 0.998x |
| polyglot_key | 1.037x | 1.005x | 23.102x |
| polyglot_recompute | 0.983x | 0.981x | 1.029x |
| repetition_key | 0.994x | 0.993x | 12.853x |
| corpus_count | 1.002x | 0.995x | 1.243x |
| corpus_materialize | 1.000x | 0.998x | 1.243x |
| corpus_uci_sorted | 1.006x | 0.999x | 0.641x |
| replay_trusted_native | 0.999x | 0.998x | 0.517x |
| replay_checked_native | 0.994x | 0.990x | 0.715x |

Cold first legal move list: 163.369 ms → 181.686 ms (0.90x).
