| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 0.999x | 0.990x | 1.082x |
| materialize | 0.999x | 0.998x | 1.176x |
| perft_bulk_d4 | 0.958x | 0.947x | 0.868x |
| perft_full_d3 | 0.836x | 0.820x | 0.509x |
| fen_parse | 1.014x | 1.005x | 1.212x |
| fen_write_native_ep | 0.996x | 0.977x | 1.046x |
| snapshot | 1.022x | 1.017x | 0.698x |
| san_parse_all | 0.952x | 0.930x | 0.880x |
| san_render_native | 0.882x | 0.854x | 1.014x |
| apply_children_full | 0.934x | 0.908x | 0.608x |
| make_unmake_full | 0.887x | 0.873x | 0.793x |
| startpos | 1.013x | 1.013x | 0.802x |
| chess960_materialize | 0.998x | 0.997x | 1.115x |
| chess960_perft_d3 | 0.955x | 0.944x | 0.805x |
| polyglot_key | 0.989x | 0.980x | 22.631x |
| polyglot_recompute | 1.001x | 1.000x | 1.133x |
| repetition_key | 0.981x | 0.976x | 13.433x |
| corpus_count | 1.008x | 1.000x | 1.208x |
| corpus_materialize | 1.007x | 1.005x | 1.120x |
| corpus_uci_sorted | 1.001x | 1.000x | 0.565x |
| replay_trusted_native | 1.031x | 0.992x | 0.330x |
| replay_checked_native | 0.921x | 0.917x | 0.612x |

Cold first legal move list: 154.021 ms → 154.087 ms (1.00x).
