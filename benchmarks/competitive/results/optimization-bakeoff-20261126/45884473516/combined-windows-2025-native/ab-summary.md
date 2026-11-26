| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.982x | 0.814x | 1.494x |
| materialize | 0.951x | 0.826x | 1.117x |
| perft_bulk_d4 | 0.971x | 0.927x | 1.398x |
| perft_full_d3 | 1.005x | 0.985x | 1.309x |
| fen_parse | 0.911x | 0.778x | 1.265x |
| fen_write_native_ep | 0.985x | 0.940x | 1.624x |
| snapshot | 0.972x | 0.863x | 1.079x |
| san_parse_all | 0.971x | 0.922x | 1.939x |
| san_render_native | 1.040x | 0.994x | 3.471x |
| apply_children_full | 0.998x | 0.942x | 1.319x |
| make_unmake_full | 0.973x | 0.918x | 1.274x |
| startpos | 0.998x | 0.998x | 1.221x |
| chess960_materialize | 1.004x | 0.994x | 1.442x |
| chess960_perft_d3 | 0.973x | 0.958x | 1.596x |
| polyglot_key | 0.931x | 0.895x | 20.377x |
| polyglot_recompute | 0.974x | 0.957x | 1.060x |
| repetition_key | 0.976x | 0.950x | 19.139x |
| corpus_count | 0.987x | 0.976x | 1.301x |
| corpus_materialize | 1.013x | 0.995x | 1.193x |
| corpus_uci_sorted | 1.018x | 1.016x | 1.506x |
| replay_trusted_native | 0.894x | 0.798x | 0.939x |
| replay_checked_native | 0.970x | 0.944x | 1.045x |

Cold first legal move list: 0.014 ms -> 0.018 ms (0.79x).
