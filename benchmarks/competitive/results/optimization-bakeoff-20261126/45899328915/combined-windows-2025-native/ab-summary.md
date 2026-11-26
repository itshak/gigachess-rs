| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.924x | 0.886x | 1.441x |
| materialize | 0.967x | 0.933x | 1.122x |
| perft_bulk_d4 | 1.005x | 0.904x | 1.442x |
| perft_full_d3 | 1.116x | 0.917x | 1.386x |
| fen_parse | 0.909x | 0.895x | 1.293x |
| fen_write_native_ep | 0.916x | 0.856x | 1.523x |
| snapshot | 0.705x | 0.660x | 0.968x |
| san_parse_all | 0.930x | 0.911x | 1.834x |
| san_render_native | 1.013x | 0.977x | 3.465x |
| apply_children_full | 0.962x | 0.895x | 1.685x |
| make_unmake_full | 0.958x | 0.933x | 1.256x |
| startpos | 0.774x | 0.774x | 0.834x |
| chess960_materialize | 0.931x | 0.918x | 1.331x |
| chess960_perft_d3 | 0.960x | 0.915x | 1.519x |
| polyglot_key | 1.030x | 0.993x | 21.087x |
| polyglot_recompute | 0.998x | 0.970x | 0.972x |
| repetition_key | 0.950x | 0.895x | 18.218x |
| corpus_count | 0.989x | 0.926x | 1.319x |
| corpus_materialize | 0.977x | 0.971x | 1.216x |
| corpus_uci_sorted | 0.980x | 0.978x | 1.466x |
| replay_trusted_native | 0.893x | 0.833x | 0.981x |
| replay_checked_native | 0.912x | 0.900x | 0.989x |

Cold first legal move list: 0.018 ms -> 0.018 ms (0.98x).
