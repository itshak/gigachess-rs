| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.957x | 0.867x | 1.384x |
| materialize | 0.975x | 0.922x | 1.209x |
| perft_bulk_d4 | 0.984x | 0.895x | 1.105x |
| perft_full_d3 | 1.003x | 0.995x | 1.440x |
| fen_parse | 1.005x | 0.893x | 1.276x |
| fen_write_native_ep | 0.961x | 0.829x | 1.622x |
| snapshot | 0.944x | 0.860x | 1.087x |
| san_parse_all | 1.011x | 0.973x | 1.890x |
| san_render_native | 0.992x | 0.963x | 4.295x |
| apply_children_full | 0.983x | 0.880x | 1.309x |
| make_unmake_full | 0.951x | 0.862x | 1.306x |
| startpos | 0.898x | 0.898x | 0.997x |
| chess960_materialize | 0.948x | 0.936x | 1.230x |
| chess960_perft_d3 | 1.008x | 0.992x | 1.418x |
| polyglot_key | 0.991x | 0.983x | 32.023x |
| polyglot_recompute | 0.931x | 0.908x | 1.081x |
| repetition_key | 0.885x | 0.849x | 14.084x |
| corpus_count | 0.996x | 0.987x | 1.004x |
| corpus_materialize | 0.992x | 0.983x | 1.119x |
| corpus_uci_sorted | 0.976x | 0.932x | 1.429x |
| replay_trusted_native | 0.930x | 0.856x | 0.761x |
| replay_checked_native | 1.020x | 1.016x | 1.038x |

Cold first legal move list: 0.021 ms -> 0.021 ms (1.00x).
