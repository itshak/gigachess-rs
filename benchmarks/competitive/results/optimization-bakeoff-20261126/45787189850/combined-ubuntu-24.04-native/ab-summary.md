| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.981x | 0.953x | 1.307x |
| materialize | 0.977x | 0.946x | 1.120x |
| perft_bulk_d4 | 1.004x | 0.987x | 1.253x |
| perft_full_d3 | 1.032x | 1.005x | 1.016x |
| fen_parse | 0.985x | 0.966x | 1.074x |
| fen_write_native_ep | 0.991x | 0.984x | 1.803x |
| snapshot | 0.818x | 0.788x | 0.827x |
| san_parse_all | 0.950x | 0.917x | 2.031x |
| san_render_native | 1.059x | 0.924x | 1.928x |
| apply_children_full | 0.820x | 0.748x | 0.885x |
| make_unmake_full | 0.855x | 0.808x | 1.018x |
| startpos | 1.096x | 1.096x | 1.201x |
| chess960_materialize | 0.992x | 0.991x | 1.424x |
| chess960_perft_d3 | 1.015x | 1.012x | 1.415x |
| polyglot_key | 0.994x | 0.990x | 20.537x |
| polyglot_recompute | 0.957x | 0.957x | 0.995x |
| repetition_key | 1.067x | 1.002x | 19.207x |
| corpus_count | 1.001x | 0.997x | 1.337x |
| corpus_materialize | 0.997x | 0.992x | 1.232x |
| corpus_uci_sorted | 1.006x | 1.004x | 1.662x |
| replay_trusted_native | 1.023x | 1.023x | 0.761x |
| replay_checked_native | 0.978x | 0.969x | 0.920x |

Cold first legal move list: 0.013 ms -> 0.012 ms (1.02x).
