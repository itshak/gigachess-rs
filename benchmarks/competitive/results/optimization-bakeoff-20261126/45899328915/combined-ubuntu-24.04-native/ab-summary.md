| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.923x | 0.891x | 1.371x |
| materialize | 0.949x | 0.929x | 1.128x |
| perft_bulk_d4 | 0.987x | 0.906x | 1.303x |
| perft_full_d3 | 0.918x | 0.821x | 1.086x |
| fen_parse | 0.917x | 0.882x | 1.206x |
| fen_write_native_ep | 0.890x | 0.827x | 1.638x |
| snapshot | 0.666x | 0.665x | 1.000x |
| san_parse_all | 0.947x | 0.928x | 2.054x |
| san_render_native | 0.954x | 0.922x | 2.003x |
| apply_children_full | 0.999x | 0.933x | 1.708x |
| make_unmake_full | 0.987x | 0.932x | 1.183x |
| startpos | 0.833x | 0.833x | 1.166x |
| chess960_materialize | 1.006x | 0.988x | 1.520x |
| chess960_perft_d3 | 0.940x | 0.937x | 1.524x |
| polyglot_key | 1.026x | 1.005x | 21.078x |
| polyglot_recompute | 0.964x | 0.962x | 1.000x |
| repetition_key | 0.948x | 0.895x | 18.196x |
| corpus_count | 0.997x | 0.942x | 1.298x |
| corpus_materialize | 0.981x | 0.975x | 1.234x |
| corpus_uci_sorted | 0.963x | 0.960x | 1.654x |
| replay_trusted_native | 0.865x | 0.738x | 0.944x |
| replay_checked_native | 0.899x | 0.893x | 1.009x |

Cold first legal move list: 0.007 ms -> 0.007 ms (1.02x).
