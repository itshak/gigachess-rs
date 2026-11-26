| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.948x | 0.753x | 1.243x |
| materialize | 0.963x | 0.797x | 1.057x |
| perft_bulk_d4 | 0.726x | 0.607x | 1.063x |
| perft_full_d3 | 0.959x | 0.813x | 1.355x |
| fen_parse | 1.022x | 0.938x | 1.184x |
| fen_write_native_ep | 0.941x | 0.814x | 1.729x |
| snapshot | 1.074x | 0.929x | 0.988x |
| san_parse_all | 0.917x | 0.814x | 1.910x |
| san_render_native | 0.986x | 0.851x | 2.754x |
| apply_children_full | 0.930x | 0.781x | 0.961x |
| make_unmake_full | 0.960x | 0.752x | 1.241x |
| startpos | 0.924x | 0.924x | 1.191x |
| chess960_materialize | 1.091x | 1.017x | 1.556x |
| chess960_perft_d3 | 0.568x | 0.515x | 1.178x |
| polyglot_key | 0.957x | 0.915x | 42.199x |
| polyglot_recompute | 0.937x | 0.866x | 1.134x |
| repetition_key | 0.938x | 0.930x | 21.567x |
| corpus_count | 0.648x | 0.644x | 1.190x |
| corpus_materialize | 0.795x | 0.778x | 1.061x |
| corpus_uci_sorted | 0.741x | 0.734x | 1.245x |
| replay_trusted_native | 0.817x | 0.689x | 0.643x |
| replay_checked_native | 0.935x | 0.907x | 1.013x |

Cold first legal move list: 0.012 ms -> 0.007 ms (1.63x).
