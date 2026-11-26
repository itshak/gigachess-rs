| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.046x | 0.780x | 1.592x |
| materialize | 0.932x | 0.751x | 1.107x |
| perft_bulk_d4 | 0.840x | 0.786x | 1.211x |
| perft_full_d3 | 1.059x | 0.862x | 1.428x |
| fen_parse | 1.066x | 0.959x | 1.271x |
| fen_write_native_ep | 0.967x | 0.691x | 1.720x |
| snapshot | 0.933x | 0.669x | 0.856x |
| san_parse_all | 0.957x | 0.777x | 1.766x |
| san_render_native | 0.856x | 0.680x | 2.626x |
| apply_children_full | 0.909x | 0.752x | 1.128x |
| make_unmake_full | 0.942x | 0.748x | 1.309x |
| startpos | 0.860x | 0.860x | 1.264x |
| chess960_materialize | 0.934x | 0.874x | 1.358x |
| chess960_perft_d3 | 0.946x | 0.932x | 1.519x |
| polyglot_key | 1.187x | 1.185x | 38.127x |
| polyglot_recompute | 1.168x | 1.139x | 1.039x |
| repetition_key | 1.125x | 1.064x | 22.034x |
| corpus_count | 1.149x | 1.103x | 1.190x |
| corpus_materialize | 1.098x | 1.011x | 1.232x |
| corpus_uci_sorted | 1.279x | 1.206x | 1.748x |
| replay_trusted_native | 0.986x | 0.841x | 0.571x |
| replay_checked_native | 1.343x | 1.296x | 1.340x |

Cold first legal move list: 0.005 ms -> 0.006 ms (0.74x).
