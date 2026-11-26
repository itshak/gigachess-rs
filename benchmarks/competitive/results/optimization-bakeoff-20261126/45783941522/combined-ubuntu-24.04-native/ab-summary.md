| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.011x | 0.996x | 1.339x |
| materialize | 0.967x | 0.949x | 1.121x |
| perft_bulk_d4 | 0.988x | 0.943x | 1.231x |
| perft_full_d3 | 1.025x | 0.955x | 1.010x |
| fen_parse | 1.108x | 1.085x | 1.218x |
| fen_write_native_ep | 0.979x | 0.886x | 1.861x |
| snapshot | 1.230x | 1.225x | 1.250x |
| san_parse_all | 1.006x | 0.999x | 2.149x |
| san_render_native | 1.004x | 0.985x | 2.191x |
| apply_children_full | 0.978x | 0.942x | 1.059x |
| make_unmake_full | 0.989x | 0.976x | 1.219x |
| startpos | 1.093x | 1.093x | 1.198x |
| chess960_materialize | 0.940x | 0.934x | 1.333x |
| chess960_perft_d3 | 1.001x | 0.991x | 1.413x |
| polyglot_key | 0.998x | 0.998x | 20.414x |
| polyglot_recompute | 0.997x | 0.986x | 1.027x |
| repetition_key | 0.930x | 0.885x | 17.747x |
| corpus_count | 1.002x | 0.996x | 1.330x |
| corpus_materialize | 1.008x | 1.005x | 1.236x |
| corpus_uci_sorted | 0.975x | 0.974x | 1.647x |
| replay_trusted_native | 0.996x | 0.976x | 0.744x |
| replay_checked_native | 1.006x | 0.995x | 1.007x |

Cold first legal move list: 0.013 ms -> 0.012 ms (1.02x).
