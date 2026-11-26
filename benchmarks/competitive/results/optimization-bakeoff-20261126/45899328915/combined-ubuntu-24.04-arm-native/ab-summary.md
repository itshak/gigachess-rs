| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.928x | 0.879x | 1.436x |
| materialize | 0.991x | 0.971x | 1.312x |
| perft_bulk_d4 | 0.980x | 0.942x | 1.361x |
| perft_full_d3 | 1.021x | 0.884x | 1.372x |
| fen_parse | 0.982x | 0.962x | 1.204x |
| fen_write_native_ep | 0.947x | 0.909x | 1.493x |
| snapshot | 1.087x | 1.080x | 0.985x |
| san_parse_all | 0.980x | 0.971x | 1.764x |
| san_render_native | 0.985x | 0.963x | 2.620x |
| apply_children_full | 1.012x | 0.967x | 1.170x |
| make_unmake_full | 0.961x | 0.908x | 1.497x |
| startpos | 0.811x | 0.811x | 0.998x |
| chess960_materialize | 0.990x | 0.990x | 1.480x |
| chess960_perft_d3 | 0.939x | 0.934x | 1.410x |
| polyglot_key | 1.048x | 1.004x | 26.643x |
| polyglot_recompute | 1.003x | 1.003x | 1.094x |
| repetition_key | 1.063x | 0.996x | 20.112x |
| corpus_count | 1.032x | 1.003x | 1.411x |
| corpus_materialize | 0.976x | 0.975x | 1.195x |
| corpus_uci_sorted | 1.004x | 1.003x | 1.463x |
| replay_trusted_native | 0.941x | 0.920x | 0.597x |
| replay_checked_native | 0.951x | 0.946x | 1.072x |

Cold first legal move list: 0.006 ms -> 0.005 ms (1.14x).
