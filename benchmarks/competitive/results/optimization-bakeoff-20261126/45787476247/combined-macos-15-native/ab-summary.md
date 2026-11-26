| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.944x | 0.710x | 1.363x |
| materialize | 0.853x | 0.585x | 0.956x |
| perft_bulk_d4 | 1.005x | 0.878x | 1.317x |
| perft_full_d3 | 0.814x | 0.618x | 1.165x |
| fen_parse | 0.914x | 0.705x | 1.166x |
| fen_write_native_ep | 0.942x | 0.771x | 1.767x |
| snapshot | 0.961x | 0.720x | 0.989x |
| san_parse_all | 0.952x | 0.740x | 1.806x |
| san_render_native | 0.948x | 0.683x | 2.601x |
| apply_children_full | 0.926x | 0.666x | 0.916x |
| make_unmake_full | 0.892x | 0.604x | 1.087x |
| startpos | 0.992x | 0.992x | 1.365x |
| chess960_materialize | 0.879x | 0.859x | 1.256x |
| chess960_perft_d3 | 0.916x | 0.871x | 1.712x |
| polyglot_key | 0.876x | 0.810x | 35.610x |
| polyglot_recompute | 0.861x | 0.811x | 0.879x |
| repetition_key | 1.006x | 0.976x | 21.045x |
| corpus_count | 0.883x | 0.757x | 1.159x |
| corpus_materialize | 0.889x | 0.862x | 1.047x |
| corpus_uci_sorted | 0.943x | 0.889x | 1.340x |
| replay_trusted_native | 0.904x | 0.881x | 0.616x |
| replay_checked_native | 0.898x | 0.799x | 0.888x |

Cold first legal move list: 0.012 ms -> 0.010 ms (1.14x).
