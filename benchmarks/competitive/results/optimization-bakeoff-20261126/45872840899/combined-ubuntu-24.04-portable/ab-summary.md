| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.013x | 0.970x | 1.245x |
| materialize | 0.988x | 0.950x | 1.341x |
| perft_bulk_d4 | 0.988x | 0.969x | 1.218x |
| perft_full_d3 | 0.978x | 0.906x | 1.323x |
| fen_parse | 0.988x | 0.949x | 1.143x |
| fen_write_native_ep | 1.018x | 0.964x | 1.439x |
| snapshot | 1.011x | 0.991x | 1.002x |
| san_parse_all | 1.012x | 0.985x | 1.793x |
| san_render_native | 1.071x | 0.981x | 2.389x |
| apply_children_full | 1.027x | 0.968x | 1.324x |
| make_unmake_full | 1.016x | 0.965x | 1.350x |
| startpos | 1.048x | 1.048x | 1.286x |
| chess960_materialize | 0.967x | 0.951x | 1.618x |
| chess960_perft_d3 | 0.976x | 0.955x | 1.533x |
| polyglot_key | 0.956x | 0.929x | 31.521x |
| polyglot_recompute | 0.961x | 0.954x | 1.005x |
| repetition_key | 1.148x | 1.083x | 24.713x |
| corpus_count | 0.977x | 0.971x | 1.078x |
| corpus_materialize | 0.969x | 0.966x | 1.214x |
| corpus_uci_sorted | 0.992x | 0.983x | 1.580x |
| replay_trusted_native | 1.022x | 0.909x | 0.766x |
| replay_checked_native | 0.968x | 0.930x | 1.250x |

Cold first legal move list: 0.007 ms -> 0.007 ms (0.97x).
