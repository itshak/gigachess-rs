| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.018x | 0.981x | 1.432x |
| materialize | 1.058x | 1.016x | 1.268x |
| perft_bulk_d4 | 1.005x | 0.957x | 1.366x |
| perft_full_d3 | 0.997x | 0.964x | 1.317x |
| fen_parse | 1.137x | 1.080x | 1.239x |
| fen_write_native_ep | 0.966x | 0.915x | 1.726x |
| snapshot | 0.988x | 0.912x | 1.205x |
| san_parse_all | 1.001x | 0.932x | 1.976x |
| san_render_native | 1.011x | 0.949x | 2.225x |
| apply_children_full | 1.032x | 0.918x | 1.336x |
| make_unmake_full | 0.963x | 0.857x | 1.238x |
| startpos | 0.957x | 0.957x | 1.253x |
| chess960_materialize | 0.987x | 0.970x | 1.431x |
| chess960_perft_d3 | 0.928x | 0.892x | 1.506x |
| polyglot_key | 0.894x | 0.879x | 28.377x |
| polyglot_recompute | 0.955x | 0.940x | 1.046x |
| repetition_key | 0.914x | 0.845x | 20.310x |
| corpus_count | 0.949x | 0.935x | 1.324x |
| corpus_materialize | 0.926x | 0.899x | 1.135x |
| corpus_uci_sorted | 0.981x | 0.933x | 1.747x |
| replay_trusted_native | 0.971x | 0.954x | 0.931x |
| replay_checked_native | 0.981x | 0.926x | 1.125x |

Cold first legal move list: 0.011 ms -> 0.012 ms (0.90x).
