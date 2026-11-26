| Workload | Candidate vs baseline | Worst case | Candidate vs fastest peer |
|---|---:|---:|---:|
| count | 1.133x | 0.980x | 1.480x |
| materialize | 1.142x | 1.049x | 1.258x |
| perft_bulk_d4 | 1.178x | 0.979x | 1.372x |
| perft_full_d3 | 1.003x | 0.930x | 1.265x |
| fen_parse | 1.146x | 1.119x | 1.284x |
| fen_write_native_ep | 1.027x | 0.965x | 1.565x |
| snapshot | 1.037x | 0.977x | 0.984x |
| san_parse_all | 1.021x | 0.946x | 2.075x |
| san_render_native | 1.008x | 0.971x | 3.751x |
| apply_children_full | 1.042x | 1.003x | 1.402x |
| make_unmake_full | 0.989x | 0.937x | 1.346x |
| startpos | 1.001x | 1.001x | 1.246x |
| chess960_materialize | 0.977x | 0.943x | 1.294x |
| chess960_perft_d3 | 1.027x | 0.988x | 1.557x |
| polyglot_key | 0.951x | 0.940x | 28.796x |
| polyglot_recompute | 1.017x | 1.011x | 1.094x |
| repetition_key | 0.979x | 0.959x | 21.942x |
| corpus_count | 1.091x | 1.058x | 1.386x |
| corpus_materialize | 1.015x | 1.002x | 1.147x |
| corpus_uci_sorted | 0.930x | 0.897x | 1.415x |
| replay_trusted_native | 0.935x | 0.924x | 0.798x |
| replay_checked_native | 0.914x | 0.862x | 0.969x |

Cold first legal move list: 0.013 ms -> 0.013 ms (0.98x).
