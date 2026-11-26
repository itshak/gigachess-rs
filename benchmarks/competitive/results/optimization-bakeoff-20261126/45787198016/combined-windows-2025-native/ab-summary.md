| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.950x | 0.919x | 1.235x |
| materialize | 0.973x | 0.932x | 1.188x |
| perft_bulk_d4 | 0.940x | 0.909x | 1.240x |
| perft_full_d3 | 0.895x | 0.861x | 1.087x |
| fen_parse | 0.887x | 0.866x | 1.116x |
| fen_write_native_ep | 0.989x | 0.982x | 1.592x |
| snapshot | 1.081x | 1.063x | 1.017x |
| san_parse_all | 0.922x | 0.899x | 1.754x |
| san_render_native | 0.831x | 0.688x | 3.036x |
| apply_children_full | 0.845x | 0.722x | 0.985x |
| make_unmake_full | 0.820x | 0.697x | 1.014x |
| startpos | 1.483x | 1.483x | 1.288x |
| chess960_materialize | 0.930x | 0.883x | 1.157x |
| chess960_perft_d3 | 0.891x | 0.830x | 1.187x |
| polyglot_key | 1.047x | 1.005x | 28.714x |
| polyglot_recompute | 1.014x | 0.967x | 1.041x |
| repetition_key | 1.214x | 1.175x | 23.623x |
| corpus_count | 0.997x | 0.987x | 1.337x |
| corpus_materialize | 0.998x | 0.976x | 1.156x |
| corpus_uci_sorted | 0.982x | 0.977x | 1.366x |
| replay_trusted_native | 0.667x | 0.554x | 0.448x |
| replay_checked_native | 0.735x | 0.623x | 0.679x |

Cold first legal move list: 0.043 ms -> 0.041 ms (1.05x).
