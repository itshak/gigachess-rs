| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.902x | 0.769x | 1.196x |
| materialize | 1.008x | 0.817x | 1.022x |
| perft_bulk_d4 | 1.033x | 0.952x | 1.266x |
| perft_full_d3 | 0.982x | 0.879x | 0.775x |
| fen_parse | 0.916x | 0.862x | 1.215x |
| fen_write_native_ep | 1.342x | 1.165x | 1.657x |
| snapshot | 1.018x | 0.910x | 0.982x |
| san_parse_all | 1.021x | 0.797x | 1.690x |
| san_render_native | 1.011x | 0.842x | 2.657x |
| apply_children_full | 1.037x | 0.945x | 0.778x |
| make_unmake_full | 1.012x | 0.938x | 1.132x |
| startpos | 1.025x | 1.025x | 1.273x |
| chess960_materialize | 1.037x | 1.011x | 1.251x |
| chess960_perft_d3 | 1.145x | 1.053x | 1.437x |
| polyglot_key | 0.942x | 0.884x | 37.032x |
| polyglot_recompute | 1.012x | 0.995x | 0.989x |
| repetition_key | 1.052x | 1.012x | 18.712x |
| corpus_count | 1.008x | 0.983x | 1.319x |
| corpus_materialize | 0.986x | 0.902x | 1.194x |
| corpus_uci_sorted | 0.933x | 0.931x | 0.919x |
| replay_trusted_native | 0.952x | 0.859x | 0.436x |
| replay_checked_native | 1.006x | 1.003x | 0.915x |

Cold first legal move list: 0.010 ms -> 0.008 ms (1.23x).
